pub fn main(init: std.process.Init) !void {
    const io = init.io;
    var buffer: [10240]u8 = undefined;
    const contents = try std.Io.Dir.readFile(std.Io.Dir.cwd(), io, "input.txt", &buffer);
    const instructions = std.mem.trim(u8, contents, " \t\r\n");

    var floor_up: usize = 0;
    var floor_down: usize = 0;
    var curr_floor: i64 = 0;
    var pos: usize = 0;
    for (instructions, 0..) |char, index| {
        move_floor(char, &floor_up, &floor_down);
        curr_floor = @as(i64, @intCast(floor_up)) - @as(i64, @intCast(floor_down));
        if (curr_floor == -1) {
            pos = index + 1;
            break;
        }
    }

    std.debug.print("up : {} down : {}\n", .{ floor_up, floor_down });
    std.debug.print("pos : {}\n", .{pos});
}

fn move_floor(char: u8, floor_up: *usize, floor_down: *usize) void {
    switch (char) {
        '(' => floor_up.* += 1,
        ')' => floor_down.* += 1,
        else => {
            std.debug.print("reached unreacheble code with : {}", .{char});
        },
    }
}

const std = @import("std");
