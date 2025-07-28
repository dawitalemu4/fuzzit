pub const CommandOutput = struct { stdout: std.ArrayListUnmanaged(u8), stderr: std.ArrayListUnmanaged(u8) };
pub const GitData = struct { status: []u8, diff: []u8 };

pub fn contains(haystack: []const u8, needle: []const u8) bool {
    return std.mem.indexOf(u8, haystack, needle) != null;
}

pub fn resolve_base_path(allocator: std.mem.Allocator) []const u8 {
    if (std.process.getEnvMap(allocator)) |env_map| {
        if (env_map.get("FUZZIT_PATH")) |custom_path| {
            return custom_path;
        } else {
            if (env_map.get("FUZZIT_BASE_PATH")) |base_path| {
                return base_path;
            } else {
                const stdin = std.io.getStdIn();
                var buf = std.io.bufferedReader(stdin.reader());
                var reader = buf.reader();

                std.debug.print("To suppress this message, add FUZZIT_BASE_PATH to your enviorinment (ex: ~/.zshrc)\n\n", .{});
                std.debug.print("Initialize FUZZIT_BASE_PATH (ex: ~/dev): ", .{});

                var input_buf: [255]u8 = undefined;
                if (reader.readUntilDelimiterOrEof(&input_buf, '\n')) |input| {
                    const input_path = input orelse {
                        std.debug.print("Invalid input provided, using ~", .{});
                        return "~";
                    };

                    if (std.fs.openDirAbsolute(input_path, .{})) |_| {
                        return input_path;
                    } else |err| {
                        std.debug.print("Invalid path provided, using ~\nErr: {}", .{err});
                        return "~";
                    }
                } else |err| {
                    std.debug.print("Path not provided, using ~\nErr: {}", .{err});
                    return "~";
                }
            }
        }
    } else |err| {
        std.debug.print("Could not get environment variables, using ~\nErr: {}", .{err});
        return "~";
    }
}

pub fn execute_os_command(allocator: std.mem.Allocator, path: []const u8, command: []const []const u8) CommandOutput {
    var child = std.process.Child.init(command, allocator);
    child.cwd = path;
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;

    var stdout: std.ArrayListUnmanaged(u8) = .empty;
    defer stdout.deinit(allocator);
    var stderr: std.ArrayListUnmanaged(u8) = .empty;
    defer stderr.deinit(allocator);

    child.spawn() catch {
        std.debug.panic("Could not execute os command", .{});
    };
    child.collectOutput(allocator, &stdout, &stderr, 255) catch |err| {
        std.debug.panic("Could not execute os command\nErr: {}", .{err});
    };
    _ = child.wait() catch |err| {
        std.debug.panic("Could not execute os command\nErr: {}", .{err});
    };

    return CommandOutput{ .stdout = stdout, .stderr = stderr };
}

pub fn collect_git_data(allocator: std.mem.Allocator, base_path: []const u8) std.StringHashMap(GitData) {
    var git_data = std.StringHashMap(GitData).init(allocator); // { "path_to_repo": { status: "...", diff: "..." } }
    defer git_data.deinit();

    _ = recursive_git_data_search(allocator, base_path, &git_data);
    return git_data;
}

pub fn recursive_git_data_search(allocator: std.mem.Allocator, current_path: []const u8, git_data: *std.StringHashMap(GitData)) std.StringHashMap(GitData) {
    const git_dir_path = std.fmt.allocPrint(allocator, "{s}/.git", .{current_path}) catch |err| {
        std.debug.panic("Invalid path error\nErr: {}", .{err});
    };
    defer allocator.free(git_dir_path);

    if (std.fs.cwd().access(git_dir_path, .{})) |_| {
        const status = execute_os_command(allocator, current_path, &[_][]const u8{ "git", "status" });
        const diff = execute_os_command(allocator, current_path, &[_][]const u8{ "git", "diff" });

        git_data.put(current_path, GitData{ .status = status.stdout.items, .diff = diff.stdout.items }) catch |err| {
            std.debug.panic("Error inserting project into git_data\nErr: {}", .{err});
        };
    } else |_| {
        var dir = std.fs.cwd().openDir(current_path, .{ .iterate = true });
        defer dir.close();

        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .directory) {
                // Skip hidden directories except .git (which we already checked)
                if (entry.name[0] == '.' and !std.mem.eql(u8, entry.name, ".git")) {
                    continue;
                }

                const subdir_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ current_path, entry.name });
                defer allocator.free(subdir_path);

                try recursive_git_data_search(allocator, subdir_path, git_data);
            }
        }
    }
}

const std = @import("std");
