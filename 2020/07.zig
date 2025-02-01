const std = @import("std");
const input = @embedFile("input");

const Bag = struct { count: u32, bag: []const u8 };

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var bags = std.StringHashMap(std.ArrayList(Bag)).init(allocator);
    defer {
        var it = bags.iterator();
        while (it.next()) |entry| {
            entry.value_ptr.deinit(); // free each ArrayList
        }
        bags.deinit();
    }

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var words = std.ArrayList([]const u8).init(allocator);
        defer words.deinit();

        var word_it = std.mem.tokenizeScalar(u8, line, ' ');
        while (word_it.next()) |word| {
            try words.append(word);
        }

        const key = mergeSlices(u8, words.items[0], words.items[1]);
        var i: usize = 4;
        while (i < words.items.len) : (i += 4) {
            if (!std.mem.eql(u8, words.items[i], "no")) { // doesn't contain any bags
                const bag = Bag{
                    .count = try std.fmt.parseInt(u32, words.items[i], 10),
                    .bag = mergeSlices(u8, words.items[i + 1], words.items[i + 2]),
                };
                const gop = try bags.getOrPut(key);
                if (gop.found_existing) {
                    try gop.value_ptr.append(bag);
                } else {
                    gop.value_ptr.* = std.ArrayList(Bag).init(allocator);
                    try gop.value_ptr.append(bag);
                }
            }
        }
    }

    try std.io.getStdOut().writer().print("Part one: {d}\nPart two: {d}\n", .{
        colorsWhichCanContain(&bags, "shiny gold"),
        bagsContained(&bags, "shiny gold"),
    });
}

/// Count the bags which can contain at some level a bag of color 'color'
fn colorsWhichCanContain(bags: *std.StringHashMap(std.ArrayList(Bag)), color: []const u8) u32 {
    var count: u32 = 0;
    var keys_it = bags.keyIterator();
    while (keys_it.next()) |key| {
        if (!std.mem.eql(u8, key.*, color) and canCarry(bags, key.*, color)) {
            count += 1;
        }
    }
    return count;
}

/// Check recursively if a bag of color 'from' can contain at least one bag of color 'to'
fn canCarry(bags: *std.StringHashMap(std.ArrayList(Bag)), from: []const u8, to: []const u8) bool {
    if (bags.contains(from)) {
        if (std.mem.eql(u8, from, to)) {
            return true;
        }
        for (bags.get(from).?.items) |bag| {
            if (canCarry(bags, bag.bag, to)) {
                return true;
            }
        }
    }
    return false;
}

/// Count the total number of individual bags contained in 'bag_color' recursively
fn bagsContained(bags: *std.StringHashMap(std.ArrayList(Bag)), bag_color: []const u8) u32 {
    var count: u32 = 0;
    if (bags.get(bag_color)) |contained_bags| {
        for (contained_bags.items) |bag| {
            count += bag.count + bag.count * bagsContained(bags, bag.bag);
        }
    }
    return count;
}

/// Merge two slices that are contiguous in memory
fn mergeSlices(T: type, slice1: []const T, slice2: []const T) []const T {
    // the +1 are due to the exclusion of the white space by std.mem.tokenizeScalar()
    std.debug.assert(@intFromPtr(slice1.ptr + slice1.len + 1) == @intFromPtr(slice2.ptr));
    return slice1.ptr[0 .. slice1.len + slice2.len + 1];
}
