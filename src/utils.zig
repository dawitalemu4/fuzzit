pub const CommandOutput = struct { stdout: std.ArrayListUnmanaged(u8), stderr: std.ArrayListUnmanaged(u8) };
pub const GitData = struct { status: []u8, diff: []u16 };

pub fn contains(haystack: []const u8, needle: []const u8) bool {
    return std.mem.indexOf(u8, haystack, needle) != null;
}

pub fn resolve_path(allocator: std.mem.Allocator) []const u8 {
    const env_map = try std.process.getEnvMap(allocator);
    defer env_map.deinit();

    if (env_map.get("FUZZIT_PATH")) |custom_path| {
        return custom_path;
    } else {
        if (env_map.get("FUZZIT_BASE_PATH")) |base_path| {
            return base_path;
        } else {
            const stdin = std.io.getStdIn();
            var buf = std.io.bufferedReader(stdin.reader());
            var reader = buf.reader();

            std.debug.print("To suppress this message, add FUZZIT_BASE_PATH to your enviorinment (ex: ~/.zshrc)\n\n");
            std.debug.print("Initialize FUZZIT_BASE_PATH (ex: ~/dev): ");

            var input_buf: [255]u8 = undefined;
            const input = try reader.readUntilDelimiterOrEof(&input_buf, '\n');

            if (input) |input_path| {
                const output = execute_os_command(allocator, input_path);

                if (output.stderr.items.len == 0) {
                    return input_path;
                } else {
                    std.debug.print("Invalid path provided, using ~");
                    return "~";
                }
            } else {
                std.debug.print("Path not provided, using ~");
                return "~";
            }
        }
    }
}

pub fn execute_os_command(allocator: std.mem.Allocator, command: [][]const u8) CommandOutput {
    var child = std.process.Child.init(command, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;

    var stdout: std.ArrayListUnmanaged(u8) = .empty;
    defer stdout.deinit(allocator);
    var stderr: std.ArrayListUnmanaged(u8) = .empty;
    defer stderr.deinit(allocator);

    try child.spawn();
    try child.collectOutput(allocator, &stdout, &stderr, 255);
    try child.wait();

    return CommandOutput{ stdout, stderr };
}

pub fn collect_git_data(allocator: std.mem.Allocator, path: []const u8, git_data: std.AutoHashMap([]const u8, GitData)) std.AutoHashMap([]const u8, GitData) {
    const cd_base = &[_][]const u8{ "cd", path };
    const ls = &[_][]const u8{ "ls", "-d", "*/", ".*/" };
    const pwd = &[_][]const u8{"pwd"};
    const git_exists = &[_][]const u8{ "[ -d .git ]", "&& echo 'y' || echo 'n'" };
    const git_status = &[_][]const u8{ "git", "status" };
    const git_diff = &[_][]const u8{ "git", "diff" };

    const base_path = execute_os_command(allocator, cd_base);
    const folders = execute_os_command(allocator, ls);
    const folders_list = folders.stdout.items;

    return git_data;
}

const std = @import("std");
