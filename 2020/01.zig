const std = @import("std");
const input = @embedFile("input");

pub fn main() !void {
    var nums: [200]u32 = undefined;
    var index: usize = 0;
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |s| : (index += 1) {
        nums[index] = try std.fmt.parseInt(u32, s, 10);
    }

    for (nums, 0..) |a, i| {
        for (nums[i + 1 ..], 0..) |b, j| {
            if (a + b == 2020) {
                std.debug.print("Part one: {d}\n", .{a * b});
            }
            for (nums[i + j + 1 ..]) |c| {
                if (a + b + c == 2020) {
                    std.debug.print("Part two: {d}\n", .{a * b * c});
                }
            }
        }
    }
}
