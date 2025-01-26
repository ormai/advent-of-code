const std = @import("std");
const input = @embedFile("input");

pub fn main() !void {
    var valid1: u32 = 0;
    var valid2: u32 = 0;

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var tokens = std.mem.tokenizeScalar(u8, line, ' ');

        var limits = std.mem.tokenizeScalar(u8, tokens.next().?, '-');
        const low = try std.fmt.parseInt(u32, limits.next().?, 10);
        const high = try std.fmt.parseInt(u32, limits.next().?, 10);
        const char = tokens.next().?[0];
        const password = tokens.next().?;

        const count = std.mem.count(u8, password, &[_]u8{char});
        if (count >= low and count <= high) {
            valid1 += 1;
        }

        if (password[low - 1] == char and password[high - 1] != char or password[low - 1] != char and password[high - 1] == char) {
            valid2 += 1;
        }
    }

    std.debug.print("Part one: {d}\nPart two: {d}\n", .{ valid1, valid2 });
}
