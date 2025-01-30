const std = @import("std");
const input = @embedFile("input");

pub fn main() void {
    const trees = [_]u32{ treesInSlope(1, 1), treesInSlope(3, 1), treesInSlope(5, 1), treesInSlope(7, 1), treesInSlope(1, 2) };
    std.debug.print("Part one: {d}\nPart two: {d}\n", .{ trees[1], multiplyAll(&trees) });
}

fn treesInSlope(right: usize, down: usize) u32 {
    var count: u32 = 0;
    var y: usize = 0;
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| : (y = (y + right) % line.len) {
        if (line[y] == '#') {
            count += 1;
        }
        for (1..down) |_| {
            _ = it.next();
        }
    }
    return count;
}

fn multiplyAll(nums: []const u32) u32 {
    if (nums.len == 0) {
        return 0;
    }
    var prod: u32 = nums[0];
    for (nums[1..]) |n| {
        prod *= n;
    }
    return prod;
}
