pub const GitData = struct { status: []u8, diff: []u16 };

fn collect_git_data(git_data: std.AutoHashMap([]const u8, GitData)) std.AutoHashMap([]const u8, GitData) {
    return git_data;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var app = yazap.App.init(allocator, "fuzzit", "Fuzzy nested git repo finder with status and diff previews");
    defer app.deinit();

    var fuzzit = app.rootCommand();
    try fuzzit.addSubcommand(app.createCommand("status", "Simple list of one-line status summaries"));

    // check for home dir in sys env, or prompt to save home path as env FUZZIT_PATH
    // check if 'PATH="" fuzzit'

    var git_data_map = std.AutoHashMap([]const u8, GitData).init(allocator); // { "path_to_repo": { status: "...", diff: "..." } }
    const git_data = collect_git_data(git_data_map);
    defer git_data_map.deinit();

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
        try diff.display();
    }
}

const std = @import("std");
const yazap = @import("yazap");

const status = @import("status.zig");
const diff = @import("diff.zig");
