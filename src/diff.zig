pub fn display(git_data: std.StringHashMap(GitData), base_path: []const u8) !void {
    _ = base_path;
    _ = git_data;
}

const std = @import("std");
const spoon = @import("spoon");

const GitData = @import("status.zig").GitData;
