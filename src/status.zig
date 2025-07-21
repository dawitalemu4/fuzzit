const green = prettyzig.RGB.init(139, 233, 255); // #8ef6ff
const yellow = prettyzig.RGB.init(241, 250, 140); // #f1fa8c
const orange = prettyzig.RGB.init(255, 184, 108); // #ffb86c
const red = prettyzig.RGB.init(255, 85, 85); // #ff5555

pub fn display(git_data: std.StringHashMap) !void {
    const stdout = std.io.getStdOut().writer();
    var git_data_iter = git_data.valueIterator();

    while (git_data_iter.next()) |data| {
        const status = data.value_ptr.status;
        try prettyzig.print(stdout, data.key_ptr + " .. ", .{});

        if (status.contains("nothing to commit")) {
            try prettyzig.print(stdout, "CLEAN\n", .{ .color = .{ .rgb = green }, .styles = &.{.italic} });
        } else if (status.contains("no changes added to commit")) {
            try prettyzig.print(stdout, "DIRTY (changes not added)\n", .{ .color = .{ .rgb = orange } });
        } else if (status.contains("Changes to be committed")) {
            try prettyzig.print(stdout, "DIRTY (changes added, not committed)\n", .{ .color = .{ .rgb = red } });
        } else if (status.contains("Your branch is ahead of")) {
            try prettyzig.print(stdout, "DIRTY (changes committed, not pushed)\n", .{ .color = .{ .rgb = red }, .styles = &.{.bold} });
        } else {
            try prettyzig.print(stdout, "UNKNOWN\n", .{ .color = .{ .rgb = yellow } });
        }
    }
}

const std = @import("std");
const prettyzig = @import("prettyzig");
