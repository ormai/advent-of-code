//! Seating arrangement problem

const std = @import("std");
const input = @embedFile("input");
const directions = [_][2]isize{
    .{ -1, -1 }, .{ -1, 0 }, .{ -1, 1 }, .{ 0, -1 },
    .{ 0, 1 },   .{ 1, -1 }, .{ 1, 0 },  .{ 1, 1 },
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var seats = std.ArrayList([]const u8).init(allocator);
    defer seats.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| try seats.append(line);

    try std.io.getStdOut().writer().print("Part one: {d}\nPart two: {d}\n", .{
        try simulate(allocator, seats.items, adjacentNeighborhood, 4),
        try simulate(allocator, seats.items, deepNeighborhood, 5),
    });
}

/// Executes the cellular atomaton until there is a generation without updates.
/// Returns the number of occupied seats after it has stopped
fn simulate(
    allocator: std.mem.Allocator,
    seats: [][]const u8,
    neighborhood: *const fn (grid: []const []const u8, row: usize, col: usize) u8,
    tolerance: u8,
) std.mem.Allocator.Error!u32 {
    const primary_grid = try gridCopy(allocator, seats);
    defer gridFree(allocator, primary_grid);

    var secondary_grid = try gridCopy(allocator, seats);
    defer gridFree(allocator, secondary_grid);

    var change = true;
    while (change) {
        change = false;
        for (0..primary_grid.len) |i| {
            for (0..primary_grid[i].len) |j| {
                switch (primary_grid[i][j]) {
                    'L' => if (neighborhood(primary_grid, i, j) == 0) {
                        secondary_grid[i][j] = '#';
                        change = true;
                    },
                    '#' => if (neighborhood(primary_grid, i, j) >= tolerance) {
                        secondary_grid[i][j] = 'L';
                        change = true;
                    },
                    else => {},
                }
            }
        }
        for (primary_grid, secondary_grid) |old, new| @memcpy(old, new);
    }

    var occupied_seats: u32 = 0;
    for (primary_grid) |row| {
        for (row) |seat| {
            if (seat == '#') occupied_seats += 1;
        }
    }
    return occupied_seats;
}

/// Count the number of neighbors 'L' of grid.items[i][j]
fn adjacentNeighborhood(grid: []const []const u8, row: usize, col: usize) u8 {
    var count: u8 = 0;
    for (directions) |dir| {
        // https://youtu.be/bApd0QFsErU
        const i: isize = @as(isize, @intCast(row)) + dir[0];
        const j: isize = @as(isize, @intCast(col)) + dir[1];
        if (i >= 0 and i < grid.len and j >= 0 and j < grid[0].len) {
            if (grid[@intCast(i)][@intCast(j)] == '#') count += 1;
        }
    }
    return count;
}

fn deepNeighborhood(grid: []const []const u8, row: usize, col: usize) u8 {
    var count: u8 = 0;
    for (directions) |dir| {
        var i: isize = @as(isize, @intCast(row)) + dir[0];
        var j: isize = @as(isize, @intCast(col)) + dir[1];
        while (i >= 0 and i < grid.len and j >= 0 and j < grid[0].len) : ({
            i += dir[0];
            j += dir[1];
        }) {
            switch (grid[@intCast(i)][@intCast(j)]) {
                '#' => {
                    count += 1;
                    break;
                },
                'L' => break,
                else => {},
            }
        }
    }
    return count;
}

fn gridCopy(
    allocator: std.mem.Allocator,
    grid: []const []const u8,
) std.mem.Allocator.Error![][]u8 {
    const grid_copy = try allocator.alloc([]u8, grid.len);
    for (grid_copy, 0..) |*row, i| row.* = try allocator.dupe(u8, grid[i]);
    return grid_copy;
}

fn gridFree(allocator: std.mem.Allocator, grid: [][]u8) void {
    for (grid) |row| allocator.free(row);
    allocator.free(grid);
}
