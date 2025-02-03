const std = @import("std");
const input = @embedFile("input");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var jolts = std.ArrayList(u32).init(allocator);
    defer jolts.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        try jolts.append(try std.fmt.parseInt(u32, line, 10));
    }

    try jolts.append(0); // charging outlet joltage
    std.mem.sort(u32, jolts.items, {}, std.sort.asc(u32));
    try jolts.append(jolts.items[jolts.items.len - 1] + 3); // device builtin adapter joltage

    try std.io.getStdOut().writer().print("Part one: {d}\nPart two: {d}\n", .{
        productOfDifferences(jolts.items),
        try distinctArrangements(allocator, jolts.items),
    });
}

/// Return the 1-jolt differences multiplied by the 3-jolt differences
fn productOfDifferences(jolts: []const u32) u32 {
    var differences = [_]u32{0} ** 4;
    for (jolts[0 .. jolts.len - 1], jolts[1..]) |a, b| {
        differences[b - a] += 1;
    }
    return differences[1] * differences[3];
}

// Returns the distinct number of ways to connect the power outlet to the device
fn distinctArrangements(allocator: std.mem.Allocator, jolts: []const u32) !u64 {
    var dp = try allocator.alloc(u64, jolts[jolts.len - 1] + 1);
    defer allocator.free(dp);
    @memset(dp[3..], 0);
    dp[0..3].* = .{ 1, 1, 2 };
    for (jolts[3..]) |jolt| {
        dp[jolt] = dp[jolt - 1] + dp[jolt - 2] + dp[jolt - 3];
    }
    return dp[jolts[jolts.len - 1]];
}
