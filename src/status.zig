const green = prettyzig.RGB.init(139, 233, 255); // #8ef6ff
const yellow = prettyzig.RGB.init(241, 250, 140); // #f1fa8c
const orange = prettyzig.RGB.init(255, 184, 108); // #ffb86c
const red = prettyzig.RGB.init(255, 85, 85); // #ff5555

fn contains(haystack: []const u8, needle: []const u8) bool {
    return std.mem.indexOf(u8, haystack, needle) != null;
}

pub fn display(git_data: std.AutoHashMap([]const u8, GitData)) !void {
    const stdout = std.io.getStdOut().writer();
    var git_data_iter = git_data.iterator();

    while (git_data_iter.next()) |data| {
        const status = data.value_ptr.status;
        var path_buffer: [256]u8 = undefined;
        const path = try std.fmt.bufPrint(&path_buffer, "{s} .. ", .{data.key_ptr.*});
        try prettyzig.print(stdout, path, .{});

        if (contains(status, "nothing to commit")) {
            try prettyzig.print(stdout, "CLEAN\n", .{ .color = .{ .rgb = green }, .styles = &.{.italic} });
        } else if (contains(status, "no changes added to commit")) {
            try prettyzig.print(stdout, "DIRTY (changes not added)\n", .{ .color = .{ .rgb = orange } });
        } else if (contains(status, "Changes to be committed")) {
            try prettyzig.print(stdout, "DIRTY (changes added, not committed)\n", .{ .color = .{ .rgb = red } });
        } else if (contains(status, "Your branch is ahead of")) {
            try prettyzig.print(stdout, "DIRTY (changes committed, not pushed)\n", .{ .color = .{ .rgb = red }, .styles = &.{.bold} });
        } else {
            try prettyzig.print(stdout, "UNKNOWN\n", .{ .color = .{ .rgb = yellow } });
        }
    }
}

const std = @import("std");
const prettyzig = @import("prettyzig");

pub const GitData = @import("main.zig").GitData;
