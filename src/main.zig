pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var app = yazap.App.init(allocator, "fuzzit", "Fuzzy nested git repo finder with status and diff previews");
    defer app.deinit();

    var fuzzit = app.rootCommand();
    try fuzzit.addSubcommand(app.createCommand("status", "Simple list of one-line status summaries"));

    var git_data_map = std.AutoHashMap([]const u8, GitData).init(allocator); // { "path_to_repo": { status: "...", diff: "..." } }
    defer git_data_map.deinit();

    const path = utils.resolve_path(allocator);
    const git_data = utils.collect_git_data(allocator, path, git_data_map);

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
        try diff.display(path);
    }
}

const std = @import("std");
const yazap = @import("yazap");

const status = @import("status.zig");
const diff = @import("diff.zig");
const utils = @import("utils.zig");

pub const GitData = @import("utils.zig").GitData;
