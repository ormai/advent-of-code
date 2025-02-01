const std = @import("std");
const input = @embedFile("input");
const window_size = 25;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var numbers = std.ArrayList(u64).init(allocator);
    defer numbers.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        try numbers.append(try std.fmt.parseInt(u64, line, 10));
    }

    const encoding_error = findEncodingError(numbers.items);
    try std.io.getStdOut().writer().print("Part one: {?d}\nPart two: {?d}\n", .{
        encoding_error,
        encryptionWeakness(numbers.items, encoding_error.?),
    });
}

fn findEncodingError(numbers: []const u64) ?u64 {
    for (numbers[window_size..], window_size..) |n, i| {
        if (out: for (numbers[i - window_size .. i]) |a| {
            for (numbers[i - window_size .. i]) |b| {
                if (a != b and a + b == n) {
                    break :out false;
                }
            }
        } else true) {
            return n;
        }
    }
    return null;
}

fn encryptionWeakness(numbers: []const u64, target: u64) ?u64 {
    for (0..numbers.len) |start| {
        for (start + 2..numbers.len + 1) |end| {
            var sum: u512 = 0;
            for (start..end) |i| {
                sum += numbers[i];
            }
            if (sum == target) {
                return std.mem.min(u64, numbers[start..end]) +
                    std.mem.max(u64, numbers[start..end]);
            }
        }
    }
    return null;
}
