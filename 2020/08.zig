const std = @import("std");
const input = @embedFile("input");

const Instruction = struct {
    arg: i32,
    op: enum { acc, jmp, nop },
    executed: bool = false,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var instructions = std.ArrayList(Instruction).init(allocator);
    defer instructions.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |i| {
        try instructions.append(Instruction{
            .op = if (std.mem.eql(u8, i[0..3], "acc")) .acc else if (std.mem.eql(u8, i[0..3], "jmp")) .jmp else .nop,
            .arg = try std.fmt.parseInt(i32, i[4..], 10),
        });
    }

    try std.io.getStdOut().writer().print(
        "Part one: {d}\nPart two: {d}\n",
        .{ execute(&instructions), fix(&instructions) },
    );
}

/// Execute the instructions and return the value of
/// the accumulator before an instruction is repated
fn execute(instructions: *std.ArrayList(Instruction)) i32 {
    var accumulator: i32 = 0;
    var program_counter: isize = 0;

    while (program_counter < instructions.items.len) {
        const instruction = instructions.items[@intCast(program_counter)];
        if (instruction.executed) {
            return accumulator;
        }
        instructions.items[@intCast(program_counter)].executed = true;
        switch (instruction.op) {
            .jmp => {
                program_counter += instruction.arg;
                continue;
            },
            .acc => accumulator += instruction.arg,
            .nop => {},
        }
        program_counter += 1;
    }

    return accumulator;
}

fn fix(instructions: *std.ArrayList(Instruction)) i32 {
    var accumulator: i32 = undefined;
    var i: usize = 0;
    while (i < instructions.items.len) : (i += 1) {
        // reset the instructions
        for (instructions.items) |*instruction| {
            instruction.executed = false;
        }
        switch (instructions.items[i].op) {
            .jmp => {
                instructions.items[i].op = .nop;
                accumulator = execute(instructions);
                instructions.items[i].op = .jmp;
            },
            .nop => {
                instructions.items[i].op = .jmp;
                accumulator = execute(instructions);
                instructions.items[i].op = .nop;
            },
            .acc => continue,
        }
        if (instructions.items[instructions.items.len - 1].executed) {
            return accumulator;
        }
    }
    return accumulator;
}
