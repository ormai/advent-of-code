const std = @import("std");
const input = @embedFile("input");

const Direction = enum {
    north,
    east,
    south,
    west,

    fn clockwise(self: *Direction, degrees: i32) void {
        self.* = @enumFromInt(@mod(@intFromEnum(self.*) + @divExact(degrees, 90), 4));
    }

    fn counterClockwise(self: *Direction, degrees: i32) void {
        self.clockwise(-degrees);
    }

    /// Static member
    fn fromChar(c: u8) Direction {
        return switch (c) {
            'N' => .north,
            'E' => .east,
            'S' => .south,
            'W' => .west,
            else => unreachable,
        };
    }
};

const Coordinates = struct {
    east_west: i32 = 0,
    north_south: i32 = 0,
    const deg_to_rad = std.math.pi / 180.0;

    fn rotate(self: *Coordinates, degrees: i32) void {
        // https://en.wikipedia.org/wiki/Rotation_of_axes_in_two_dimensions#math_5
        const angle: f32 = @as(f32, @floatFromInt(degrees)) * deg_to_rad;
        const x: f32 = @floatFromInt(self.east_west);
        const y: f32 = @floatFromInt(self.north_south);
        self.east_west = @intFromFloat(@round(x * @cos(angle) + y * @sin(angle)));
        self.north_south = @intFromFloat(@round(-x * @sin(angle) + y * @cos(angle)));
    }
};

pub fn main() !void {
    try std.io.getStdOut().writer().print("Part one: {d}\nPart two: {d}\n", .{
        try shipDistance(),
        try waypointDistance(),
    });
}

fn shipDistance() !u32 {
    var direction = Direction.east;
    var ship: Coordinates = .{};
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const action, const value = .{ line[0], try std.fmt.parseInt(i32, line[1..], 10) };
        switch (action) {
            'L' => direction.counterClockwise(value),
            'R' => direction.clockwise(value),
            else => switch (if (action == 'F') direction else Direction.fromChar(action)) {
                .north => ship.north_south += value,
                .east => ship.east_west += value,
                .south => ship.north_south -= value,
                .west => ship.east_west -= value,
            },
        }
    }
    return @abs(ship.east_west) + @abs(ship.north_south);
}

fn waypointDistance() !u32 {
    var ship = Coordinates{};
    var waypoint: Coordinates = .{ .east_west = 10, .north_south = 1 };
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const action, const value = .{ line[0], try std.fmt.parseInt(i32, line[1..], 10) };
        switch (action) {
            'F' => {
                ship.east_west += waypoint.east_west * value;
                ship.north_south += waypoint.north_south * value;
            },
            'N' => waypoint.north_south += value,
            'S' => waypoint.north_south -= value,
            'E' => waypoint.east_west += value,
            'W' => waypoint.east_west -= value,
            'L' => waypoint.rotate(-value),
            'R' => waypoint.rotate(value),
            else => unreachable,
        }
    }
    return @abs(ship.east_west) + @abs(ship.north_south);
}
