const std = @import("std");
const input = @embedFile("input");

pub fn main() !void {
    var ids = std.ArrayList(i32).init(std.heap.page_allocator);
    defer ids.deinit();

    var highest_seat_id: i32 = 0;
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |seat| {
        var row_low: i32 = 0;
        var row_high: i32 = 128;
        for (seat[0..7]) |c| {
            if (c == 'B') {
                row_low += @divFloor(row_high - row_low, 2);
            } else {
                row_high -= @divFloor(row_high - row_low, 2);
            }
        }

        var col_low: i32 = 0;
        var col_high: i32 = 8;
        for (seat[7..]) |c| {
            if (c == 'R') {
                col_low += @divFloor(col_high - col_low, 2);
            } else {
                col_high -= @divFloor(col_high - col_low, 2);
            }
        }

        const seat_id = row_low * 8 + col_low;
        if (seat_id > highest_seat_id) {
            highest_seat_id = seat_id;
        }
        try ids.append(seat_id);
    }
    std.debug.print("Part one: {d}\n", .{highest_seat_id});

    std.mem.sort(i32, ids.items, {}, std.sort.asc(i32));
    var win_it = std.mem.window(i32, ids.items, 2, 1);
    while (win_it.next()) |win| {
        if (win[1] - win[0] == 2) {
            std.debug.print("Part two: {d}\n", .{win[0] + 1});
        }
    }
}
