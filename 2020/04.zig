const std = @import("std");
const input = @embedFile("input");

pub fn main() void {
    std.debug.print("Part one: {d}\nPart two: {d}\n", .{ partOne(), partTwo() });
}

// Counts the number of valid passports
fn partOne() u32 {
    var valid: u32 = 0;
    var it = std.mem.tokenizeSequence(u8, input, "\n\n");
    while (it.next()) |passport| {
        var fields: u32 = 0;
        var field_it = std.mem.tokenizeAny(u8, passport, " \n");
        while (field_it.next()) |passport_field| {
            for ([_][]const u8{ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" }) |field| {
                if (std.mem.eql(u8, passport_field[0..3], field)) {
                    fields += 1;
                }
            }
        }
        if (fields == 7) {
            valid += 1;
        }
    }
    return valid;
}

// Counts the number of valid passports with additional checks
fn partTwo() u32 {
    var valid_passports: u32 = 0;
    var it = std.mem.tokenizeSequence(u8, input, "\n\n");
    while (it.next()) |passport| {
        var fields: u32 = 0;
        var field_it = std.mem.tokenizeAny(u8, passport, "\n ");
        while (field_it.next()) |passport_field| {
            if (std.mem.eql(u8, passport_field[0..3], "byr")) {
                const year = std.fmt.parseInt(u32, passport_field[4..8], 10) catch break;
                if (year >= 1920 and year <= 2002) {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "iyr")) {
                const year = std.fmt.parseInt(u32, passport_field[4..8], 10) catch break;
                if (year >= 2010 and year <= 2020) {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "eyr")) {
                const year = std.fmt.parseInt(u32, passport_field[4..8], 10) catch break;
                if (year >= 2020 and year <= 2030) {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "hgt")) {
                const unit = passport_field[passport_field.len - 2 ..];
                const height = std.fmt.parseInt(u32, passport_field[4 .. passport_field.len - 2], 10) catch break;
                if (std.mem.eql(u8, unit, "cm") and height >= 150 and height <= 193 or
                    std.mem.eql(u8, unit, "in") and height >= 59 and height <= 76)
                {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "hcl")) {
                if (passport_field[4] == '#' and passport_field[5..].len == 6 and
                    for (passport_field[5..11]) |c|
                {
                    if (!std.ascii.isHex(c)) {
                        break false;
                    }
                } else true) {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "ecl")) {
                if (std.mem.eql(u8, passport_field[4..], "amb") or std.mem.eql(u8, passport_field[4..], "blu") or
                    std.mem.eql(u8, passport_field[4..], "brn") or std.mem.eql(u8, passport_field[4..], "gry") or
                    std.mem.eql(u8, passport_field[4..], "grn") or std.mem.eql(u8, passport_field[4..], "hzl") or
                    std.mem.eql(u8, passport_field[4..], "oth"))
                {
                    fields += 1;
                }
            } else if (std.mem.eql(u8, passport_field[0..3], "pid")) {
                if (passport_field[4..].len == 9 and for (passport_field[4..10]) |c| {
                    if (!std.ascii.isDigit(c)) {
                        break false;
                    }
                } else true) {
                    fields += 1;
                }
            }
        }
        if (fields == 7) {
            valid_passports += 1;
        }
    }
    return valid_passports;
}
