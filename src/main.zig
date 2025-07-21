const SubCommands = enum {
    help,
    status,
};

const main_parsers = .{
    .command = clap.parsers.enumeration(SubCommands),
};

// The parameters for `main`. Parameters for the subcommands are specified further down.
const main_params = clap.parseParamsComptime(
    \\-h, --help  Display this help and exit.
    \\status      Simple list of one-line status summaries
    \\
);

fn collect_git_data(gpa: std.heap.SmpAllocator) std.StringHashMap {
    const GitData = struct { status: []u16, diff: []u16 };
    var git_data = std.StringHashMap(GitData).init(gpa); // { "path": { status: "...", diff: "..." } }
    defer git_data.deinit();
}

pub fn main() !void {
    var gpa_state = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = gpa_state.allocator();
    defer _ = gpa_state.deinit();

    var iter = try std.process.ArgIterator.initWithAllocator(gpa);
    defer iter.deinit();
    _ = iter.next();

    var diag = clap.Diagnostic{};
    var res = clap.parseEx(clap.Help, &main_params, main_parsers, &iter, .{
        .diagnostic = &diag,
        .allocator = gpa,

        // Terminate the parsing of arguments after parsing the first positional (0 is passed
        // here because parsed positionals are, like slices and arrays, indexed starting at 0).
        //
        // This will terminate the parsing after parsing the subcommand enum and leave `iter`
        // not fully consumed. It can then be reused to parse the arguments for subcommands.
        .terminating_positional = 0,
    }) catch |err| {
        diag.report(std.io.getStdErr().writer(), err) catch {};
        return err;
    };
    defer res.deinit();

    if (res.args.help != 0) std.debug.print("--help\n", .{});

    const git_data = collect_git_data(gpa);
    const command = res.positionals[0] orelse return error.MissingCommand;
    switch (command) {
        .h => std.debug.print("--help\n", .{}),
        .help => std.debug.print("--help\n", .{}),
        .status => {
            status.display(git_data);
        },
        else => {
            diff.display(git_data);
        },
    }
}

const std = @import("std");
const clap = @import("clap");

const status = @import("status");
const diff = @import("diff");
