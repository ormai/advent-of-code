const std = @import("std");
const input = @embedFile("input");

pub fn main() !void {
    var count_any: u32 = 0;
    var count_every: u32 = 0;
    var it = std.mem.tokenizeSequence(u8, input, "\n\n");
    while (it.next()) |group| {
        var freqs = [_]u8{0} ** 26;

        var persons: u32 = 0;
        var person_it = std.mem.tokenizeScalar(u8, group, '\n');
        while (person_it.next()) |person| {
            persons += 1;
            for (person) |c| {
                if (std.ascii.isAlphabetic(c)) {
                    freqs[c - 'a'] += 1;
                }
            }
        }

        for (freqs) |freq| {
            if (freq > 0) {
                count_any += 1;
            }
            if (freq == persons) {
                count_every += 1;
            }
        }
    }

    try std.io.getStdOut().writer().print("Part one: {d}\nPart two: {d}\n", .{ count_any, count_every });
}
