pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var app = yazap.App.init(allocator, "fuzzit", "Fuzzy nested git repo finder with status and diff previews");
    defer app.deinit();

    var fuzzit = app.rootCommand();
    try fuzzit.addSubcommand(app.createCommand("status", "Simple list of one-line status summaries"));

    const base_path = utils.resolve_base_path(allocator);
    const git_data = utils.collect_git_data(allocator, base_path);

    const input = try app.parseProcess();
    if (input.subcommandMatches("help")) |_| {
        try app.displayHelp();
    } else if (input.containsArg("help")) {
        try app.displayHelp();
    } else if (input.containsArg("h")) {
        try app.displayHelp();
    }

    if (input.subcommandMatches("status")) |_| {
        try status.display(git_data);
    } else {
        try diff.display(git_data, base_path);
    }
}

const std = @import("std");
const yazap = @import("yazap");

const status = @import("status.zig");
const diff = @import("diff.zig");
const utils = @import("utils.zig");

pub const GitData = @import("utils.zig").GitData;
