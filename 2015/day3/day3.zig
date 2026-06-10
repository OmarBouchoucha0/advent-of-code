const std = @import("std");

const Node = struct {
    up: ?*Node,
    down: ?*Node,
    left: ?*Node,
    right: ?*Node,
    visited: bool,
};

fn create_node() Node {
    return Node{
        .up = null,
        .down = null,
        .left = null,
        .right = null,
        .visited = false,
    };
}

fn visitNode(node: *Node) void {
    node.visited = true;
}

fn goUp(from: *Node, to: *Node) void {
    from.up = to;
}

fn goDown(from: *Node, to: *Node) void {
    from.down = to;
}

fn goLeft(from: *Node, to: *Node) void {
    from.left = to;
}

fn goRight(from: *Node, to: *Node) void {
    from.right = to;
}

fn createGraph(input: []const u8, start: *Node) void {
    if (input.len == 0) {
        return;
    }
    // inital starting point
    var curr_node = start;
    for (input) |direction| {
        // this has to also mutate the new destination node because we can go back
        curr_node.visited = true;
        switch (direction) {
            '^' => {
                if (curr_node.up == null) {
                    var destination: Node = create_node();
                    goUp(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goUp(curr_node, curr_node.up.?);
                    curr_node = curr_node.up.?;
                }
            },
            'v' => {
                if (curr_node.down == null) {
                    var destination: Node = create_node();
                    goDown(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goDown(curr_node, curr_node.down.?);
                    curr_node = curr_node.down.?;
                }
            },
            '<' => {
                if (curr_node.left == null) {
                    var destination: Node = create_node();
                    goLeft(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goLeft(curr_node, curr_node.left.?);
                    curr_node = curr_node.left.?;
                }
            },
            '>' => {
                if (curr_node.right == null) {
                    var destination: Node = create_node();
                    goRight(curr_node, &destination);
                    curr_node = &destination;
                } else {
                    goRight(curr_node, curr_node.right.?);
                    curr_node = curr_node.right.?;
                }
            },
            else => {
                unreachable;
            },
        }
    }
}

pub fn main() !void {
    // const io = init.io;
    // var buffer: [10240]u8 = undefined;
    // const contents = try std.Io.Dir.readFile(std.Io.Dir.cwd(), io, "input.txt", &buffer);
    // const trimmed = std.mem.trim(u8, contents, " \t\r\n");
    // const lines = std.mem.splitSequence(u8, trimmed, "\n");
    std.debug.print("hello world!", .{});
}

test "empty graph" {
    const input: []u8 = "";
    var start = create_node();
    createGraph(input, &start);
    try std.testing.expect(start.right == null);
    try std.testing.expect(start.down == null);
    try std.testing.expect(start.left == null);
    try std.testing.expect(start.up == null);
}

test "graph of 1" {
    const input = ">";
    var start = create_node();
    createGraph(input, &start);
    try std.testing.expect(start.right != null);
}

test "graph of 3" {
    const input = ">^<";
    var start = create_node();
    createGraph(input, &start);
    try std.testing.expect(start.right != null);
}
