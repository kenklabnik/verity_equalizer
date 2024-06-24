//just realized an easier way to implement some of the logic compared to my first attempt

#[derive(PartialEq, Copy, Clone, Debug)]
enum Shapes {
    Circle,
    Square,
    Triangle,
}

impl Shapes {
    fn as_str(&self) -> &'static str {
        match self {
            Shapes::Circle => "Circle",
            Shapes::Square => "Square",
            Shapes::Triangle => "Triangle",
        }
    }
}

struct Room {
    statue_shape: Shapes,
    room_shapes: Vec<Shapes>,
}

impl Room {

    fn new(statue: Shapes) -> Room {
        Room {
            statue_shape: statue,
            room_shapes: Vec::new(),
        }
    }

    fn setup(&mut self, shape_1: Shapes, shape_2: Shapes) {
        self.room_shapes.clear();
        self.room_shapes.push(shape_1);
        self.room_shapes.push(shape_2);
    }

    fn has(&self, query_shape: Shapes) -> bool {
        //Because vecs work on FILO, we only need to check the first two shapes.
        //If there are more, accidentally checking them could cause logic errors later.
        if self.room_shapes.len() >= 2 {
            return self.room_shapes[0] == query_shape || self.room_shapes[1] == query_shape;
        }
        false
    }

    fn dunk_on(&mut self, shape: Shapes, receiver: &mut Room) {
        let mut i = 0;
        for item in &self.room_shapes {
            if &shape == item {
                let dunk_shape = shape.clone();
                receiver.room_shapes.push(dunk_shape.clone());

                let mut result = String::new();
                result.push_str(&self.statue_shape.as_str());
                result.push_str(" player dunks ");
                result.push_str(dunk_shape.as_str());
                result.push_str(" on statue with ");
                result.push_str(&receiver.statue_shape.as_str());
                println!("{}", result);
                break;
            } 
            i+=1;
        }
        if i <= self.room_shapes.len() {
            self.room_shapes.remove(i);
        }

    }
}

fn main() {
    println!("\nVerity Equal Distribution Simulator");
    println!("To escape: dunk both of your starting shapes to give them to other players. When everyone has one copy of each shape their statue isn't holding, you've succeeded.");
    println!("(Also, no statue can be dunked on twice in a row. Also, outside has to do stuff.)\n");

    //TODO take input for starting state, e.g. CST, TCS as well as room shadows
    //TODO limit input/output for just one player's view; multithreading probably simulates this nicely

    let mut circle_room = Room::new(Shapes::Circle);
    let mut square_room = Room::new(Shapes::Square);
    let mut triangle_room = Room::new(Shapes::Triangle);

    println!("Case 1: Circle has two circles, Square has two squares, Triangle has two triangles:\n");

    circle_room.setup(Shapes::Circle, Shapes::Circle);
    square_room.setup(Shapes::Square, Shapes::Square);
    triangle_room.setup(Shapes::Triangle, Shapes::Triangle);
    solve(&mut circle_room, &mut square_room, &mut triangle_room);

    println!("Case 2: Circle has two circles, Square has two triangles, Triangle has two squares:\n");

    circle_room.setup(Shapes::Circle, Shapes::Circle);
    square_room.setup(Shapes::Triangle, Shapes::Triangle);
    triangle_room.setup(Shapes::Square, Shapes::Square);
    solve(&mut circle_room, &mut square_room, &mut triangle_room);

    println!("Case 3: Circle has CT, Square has CS, Triangle has TS:\n");

    circle_room.setup(Shapes::Circle, Shapes::Triangle);
    square_room.setup(Shapes::Circle, Shapes::Square);
    triangle_room.setup(Shapes::Triangle, Shapes::Square);
    solve(&mut circle_room, &mut square_room, &mut triangle_room);

    println!("Case 4: Circle has two squares, Square has two triangles, Triangle has two circles:\n");
    circle_room.setup(Shapes::Square, Shapes::Square);
    square_room.setup(Shapes::Triangle, Shapes::Triangle);
    triangle_room.setup(Shapes::Circle, Shapes::Circle);
    solve(&mut circle_room, &mut square_room, &mut triangle_room);

    println!("Case 5: Circle has two circles, Square has ST, Triangle has ST:\n");
    circle_room.setup(Shapes::Circle, Shapes::Circle);
    square_room.setup(Shapes::Square, Shapes::Triangle);
    triangle_room.setup(Shapes::Triangle, Shapes::Square);
    solve(&mut circle_room, &mut square_room, &mut triangle_room);
}

fn solve(circle_room: &mut Room, square_room: &mut Room, triangle_room: &mut Room) {
    let mut dunk_counter = 0;

    //this will help us later; see "Back Half Bug" comments
    let matching_start = circle_room.has(Shapes::Circle) || square_room.has(Shapes::Square) || triangle_room.has(Shapes::Triangle);

    //first dunk per player
    //circle dunks to square, square dunks to triangle, triangle dunks to circle

    //circle room: determine current state
    match(circle_room.room_shapes[0], circle_room.room_shapes[1]) {
        (x, y) if x == y => {
            //Both shapes are the same; dunk on square first
            circle_room.dunk_on(x.clone(), square_room);
        }
        (x, y) if x == Shapes::Square || y == Shapes::Square => {
            //Both shapes are different; the unmatching is a square, so dunk circle on square first
            circle_room.dunk_on(Shapes::Circle, square_room);
        }
        _ => {
            //Both shapes are different; the unmatching is a triangle, so dunk triangle on square first
            circle_room.dunk_on(Shapes::Triangle, square_room);
        }
    }
    dunk_counter += 1;

    //square room: determine current state
    match(square_room.room_shapes[0], square_room.room_shapes[1]) {
        (x, y) if x == y => {
            //Both shapes are the same; dunk on triangle first
            square_room.dunk_on(x.clone(), triangle_room);
        }
        (x, y) if x == Shapes::Triangle || y == Shapes::Triangle => {
            //Both shapes are different; the unmatching is a triangle, so dunk square on triangle first
            square_room.dunk_on(Shapes::Square, triangle_room);
        }
        _ => {
            //Both shapes are different; the unmatching is a circle, so dunk circle on triangle first
            square_room.dunk_on(Shapes::Circle, triangle_room);
        }
    }
    dunk_counter += 1;


    //triangle room: determine current state
    match(triangle_room.room_shapes[0], triangle_room.room_shapes[1]) {
        (x, y) if x == y => {
            //Both shapes are the same; dunk on circle first
            triangle_room.dunk_on(x.clone(), circle_room);
        }
        (x, y) if x == Shapes::Circle || y == Shapes::Circle => {
            //Both shapes are different; the unmatching is a circle, so dunk triangle on circle first
            triangle_room.dunk_on(Shapes::Triangle, circle_room);
        }
        _ => {
            //Both shapes are different; the unmatching is a square, so dunk square on circle first
            triangle_room.dunk_on(Shapes::Square, circle_room);
        }
    }
    dunk_counter += 1;

    println!("First set of dunks completed. Dunk counter: {}\n", dunk_counter);

    //second dunk per player
    //circle dunks to triangle, triangle dunks to square, square dunks to circle
    //conveniently for this program, the initial shape that isn't yet dunked will be at position zero in the Vec no matter what

    circle_room.dunk_on(circle_room.room_shapes[0].clone(), triangle_room);
    triangle_room.dunk_on(triangle_room.room_shapes[0].clone(), square_room);
    square_room.dunk_on(square_room.room_shapes[0].clone(), circle_room);
    dunk_counter += 3;

    println!("Second set of dunks completed. Dunk counter: {}\n", dunk_counter);

    println!("State after two sets of dunks:");
    println!("Circle Room has {} and {}", circle_room.room_shapes[0].clone().as_str(), circle_room.room_shapes[1].clone().as_str());
    println!("Square Room has {} and {}", square_room.room_shapes[0].clone().as_str(), square_room.room_shapes[1].clone().as_str());
    println!("Triangle Room has {} and {}\n", triangle_room.room_shapes[0].clone().as_str(), triangle_room.room_shapes[1].clone().as_str());

    let mut last_statue_shape = Shapes::Circle;

    //at this point the possibilities, given no errors, are:

    //for triple matching:
    //C=CC, S=SS, T=TT      ->      C=ST, S=CT, T=CS (SOLVED)

    //for triple unmatching:
    //C=SS, S=TT, T=CC      ->      C=CT, S=CS, T=ST ("triple half-matching")
    //C=TT, S=CC, T=SS      ->      C=CS, S=CT, T=CS ("triple half-matching")

    //for triple half-matching:
    //C=CS, S=ST, T=CT      ->      C=TT, S=CC, T=SS ("triple unmatching")
    //C=CT, S=CS, T=ST      ->      C=SS, S=TT, T=CC ("triple unmatching")

    //for matching-half-half:
    //C=CC, S=ST, T=ST      ->      C=ST, S=CT, T=CS (SOLVED) 
    //C=CT, S=SS, T=CT      ->      C=ST, S=CT, T=CS (SOLVED) 
    //C=CS, S=CS, T=TT      ->      C=ST, S=CT, T=CS (SOLVED)

    //for matching-unmatching-unmatching:
    //C=CC, S=TT, T=SS      ->      C=ST, S=CS, T=CT ("exit-half-half", C can exit)
    //C=TT, S=SS, T=CC      ->      C=CS, S=CT, T=CS ("exit-half-half", S can exit)
    //C=SS, S=CC, T=TT      ->      C=CT, S=ST, T=CS ("exit-half-half", T can exit)

    //Back Half Bug:
    //When the ending configuration is Triple Half, the extra swaps that need to happen are different from when it's exit-half-half.
    //For C=CT, S=CS, T=ST: C needs to dunk circle to T.
    //For C=CT, S=ST, T=CS: T is solved, so C needs to dunk circle to S.
    //Likewise:
    //For C=CS, S=ST, T=CT: C need to dunk circle to S.
    //For C=CS, S=CT, T=ST: S is solved, so C needs to dunk circle to T.
    //These situations appear identical for the circle player.
    //Therefore a callout has to happen at some point.
    //The easiest way is this: the "exit-half-half" situation only happens when one player starts matching and the other two start unmatching.
    //If everyone inside knows to call it out if they have a matching start, and exactly one person does so, then:
    //C now knows if they are half-matching at the end, they dunk on the player their other symbol DOESN'T match.
    //If that callout doesn't happen, then the start was triple-unmatching, so C can dunk on the player their other symbol DOES match.

    //from here, there should only be at most one dunk per player required, but I'm putting it in a loop for reasons
    let mut c_solved: bool;
    let mut s_solved: bool;
    let mut t_solved: bool;


   loop {
        //is circle solved?
        if circle_room.has(Shapes::Square) && circle_room.has(Shapes::Triangle) && circle_room.room_shapes.len() == 2 {
            c_solved = true;
        } else {
            c_solved = false; //I believe this should never happen, but might as well
        }

        //is square solved?
        if square_room.has(Shapes::Triangle) && square_room.has(Shapes::Circle) && square_room.room_shapes.len() == 2 {
            s_solved = true;
        } else {
            s_solved = false; 
        }

        //is triangle solved?
        if triangle_room.has(Shapes::Circle) && triangle_room.has(Shapes::Square) && triangle_room.room_shapes.len() == 2 {
            t_solved = true;
        } else {
            t_solved = false; 
        }

        //are all three rooms solved?
        if c_solved && s_solved && t_solved {
            break;
        }

        //if it's circle's turn:
        if !c_solved && last_statue_shape == Shapes::Circle {
            //determine if half-matching or unmatching; should never be matching at this point in the encounter, and exit behavior isn't in this code block
            if circle_room.room_shapes[0] == circle_room.room_shapes[1] {
                //Unmatching; dunk either copy of that shape on the statue that needs that shape to exit
                if circle_room.has(Shapes::Square) {
                    circle_room.dunk_on(Shapes::Square, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                } else if circle_room.has(Shapes::Triangle) {
                    circle_room.dunk_on(Shapes::Triangle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                }
            } else if matching_start {
                //if a player in this phase is half-matching, it depends on the start state:
                //if somebody called "matching start", the person who needs their matching shape is the one who DOESN'T match their other shape
                //otherwise, dunk the matching shape on the person who DOES match it
                println!("Since it was a matching start, ");
                if circle_room.has(Shapes::Square) {
                    circle_room.dunk_on(Shapes::Circle, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                } else if circle_room.has(Shapes::Triangle) {
                    circle_room.dunk_on(Shapes::Circle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                }
            } else {
                if circle_room.has(Shapes::Square) {
                    circle_room.dunk_on(Shapes::Circle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                } else if circle_room.has(Shapes::Triangle) {
                    circle_room.dunk_on(Shapes::Circle, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                }
            }
        } else if c_solved && last_statue_shape == Shapes::Circle && (!s_solved || !t_solved) {
            //for the situation where c is the most recent dunk, is ready to exit, but s and t aren't ready to exit

            println!("Circle statue is the most recent dunk, but that player has no valid moves. Passing turn to square.");
            last_statue_shape = Shapes::Square;
        }

        //if it's square's turn:
        if !s_solved && last_statue_shape == Shapes::Square {
            //determine if half-matching or unmatching; should never be matching at this point in the encounter, and exit behavior isn't in this code block
            if square_room.room_shapes[0] == square_room.room_shapes[1] {
                //Unmatching; dunk either copy of that shape on the statue that needs that shape to exit
                if square_room.has(Shapes::Circle) {
                    square_room.dunk_on(Shapes::Circle, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                } else if square_room.has(Shapes::Triangle) {
                    square_room.dunk_on(Shapes::Triangle, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                }
            } else if matching_start {
                //if a player in this phase is half-matching, it depends on the start state:
                //if somebody called "matching start", the person who needs their matching shape is the one who DOESN'T match their other shape
                //otherwise, dunk the matching shape on the person who DOES match it
                println!("Since it was a matching start, ");
                if square_room.has(Shapes::Triangle) {
                    square_room.dunk_on(Shapes::Square, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                } else if square_room.has(Shapes::Circle) {
                    square_room.dunk_on(Shapes::Square, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                }
            } else {
                if square_room.has(Shapes::Circle) {
                    square_room.dunk_on(Shapes::Square, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                } else if square_room.has(Shapes::Triangle) {
                    square_room.dunk_on(Shapes::Square, triangle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Triangle;
                }
            }
        } //There shouldn't be a case where it's Square player's turn but their room is solved, since this loop checks for everyone being solved before getting here
          //and if Square is solved but the other two aren't, Circle's code will go first and pass the turn to Triangle

        //if it's triangle's turn:
        if !t_solved && last_statue_shape == Shapes::Triangle {
            //determine if half-matching or unmatching; should never be matching at this point in the encounter, and exit behavior isn't in this code block
            if triangle_room.room_shapes[0] == triangle_room.room_shapes[1] {
                //Unmatching; dunk either copy of that shape on the statue that needs that shape to exit
                if triangle_room.has(Shapes::Circle) {
                    triangle_room.dunk_on(Shapes::Circle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                } else if triangle_room.has(Shapes::Square) {
                    triangle_room.dunk_on(Shapes::Triangle, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                }
            } else if matching_start {
                //if a player in this phase is half-matching, it depends on the starting state as seen in the other two
                println!("Since it was a matching start, ");
                if triangle_room.has(Shapes::Square) {
                    triangle_room.dunk_on(Shapes::Triangle, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                } else if triangle_room.has(Shapes::Circle) {
                    triangle_room.dunk_on(Shapes::Triangle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                }
            } else {
                if triangle_room.has(Shapes::Circle) {
                    triangle_room.dunk_on(Shapes::Triangle, circle_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Circle;
                } else if triangle_room.has(Shapes::Square) {
                    triangle_room.dunk_on(Shapes::Triangle, square_room);
                    dunk_counter += 1;
                    last_statue_shape = Shapes::Square;
                }
            }
        }
    }
    println!("GG! Now tell outside to do their thing, starting on any statue but {}", last_statue_shape.as_str());
    println!("Total dunks: {}\n---\n", dunk_counter);
}
