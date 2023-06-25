// main file
fn main() {
    println!("Welcome to the Adventure Park!");
}


// Zip lining
struct ZipLine {
    length: u32,
    max_speed: u32,
    safety_rating: u8,
}

impl ZipLine {
    pub fn new(length: u32, max_speed: u32, safety_rating: u8) -> ZipLine {
        ZipLine {
            length,
            max_speed,
            safety_rating,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn max_speed(&self) -> u32 {
        self.max_speed
    }

    pub fn safety_rating(&self) -> u8 {
        self.safety_rating
    }
}

// Ropes Courses
struct RopesCourse {
    difficulty: u8,
    obstacles: u8,
    safety_rating: u8,
}

impl RopesCourse {
    pub fn new(difficulty: u8, obstacles: u8, safety_rating: u8) -> RopesCourse {
        RopesCourse {
            difficulty,
            obstacles,
            safety_rating,
        }
    }
    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn obstacles(&self) -> u8 {
        self.obstacles
    }

    pub fn safety_rating(&self) -> u8 {
        self.safety_rating
    }
}

// Other Aerial Challenges
struct AerialChallenge {
    height: u32,
    difficulty: u8,
    safety_rating: u8,
}

impl AerialChallenge {
    pub fn new(height: u32, difficulty: u8, safety_rating: u8) -> AerialChallenge {
        AerialChallenge {
            height,
            difficulty,
            safety_rating,
        }
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn safety_rating(&self) -> u8 {
        self.safety_rating
    }
}

// Thrill Seekers
struct ThrillSeeker {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    thrill_level: u8,
}

impl ThrillSeeker {
    pub fn new(name: String, age: u8, height: u32, weight: u32, thrill_level: u8) -> ThrillSeeker {
        ThrillSeeker {
            name,
            age,
            height,
            weight,
            thrill_level,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn thrill_level(&self) -> u8 {
        self.thrill_level
    }
}