enum Emotion {
    Anger,
    Happy
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion,
}

struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
 }

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }

    fn tell_state(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut p = HappyPerson {
        name: "Okumura".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());
    {
        let d = Droppable;
    }
    println!("The Droppable should be released at the end of block.");
}
