use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%&()_-+=?<>[]{}|~^";

const WORD_LIST: &[&str] = &[
    "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd",
    "abuse", "access", "acid", "across", "action", "actor", "adapt", "address", "adjust",
    "admit", "adult", "advance", "advice", "afraid", "again", "agent", "agree", "ahead",
    "alarm", "album", "alien", "almost", "alone", "alpha", "already", "alter", "always",
    "amateur", "amazing", "among", "amount", "anchor", "ancient", "angel", "anger", "angle",
    "animal", "ankle", "annual", "answer", "antenna", "apart", "apple", "april", "arena",
    "argue", "armor", "army", "arrow", "artist", "atom", "august", "aunt", "autumn",
    "average", "avoid", "awake", "aware", "awful", "axis", "baby", "bacon", "badge",
    "bamboo", "banana", "banner", "barely", "barrel", "basic", "basket", "battle", "beach",
    "beard", "become", "before", "begin", "behind", "believe", "below", "bench", "best",
    "betray", "beyond", "bicycle", "bird", "bitter", "blade", "blame", "blanket", "blast",
    "blaze", "bleak", "blend", "blind", "block", "blood", "blossom", "blue", "board",
    "bomb", "bone", "bonus", "book", "border", "boss", "bottom", "bounce", "bowl",
    "brave", "bread", "breeze", "brick", "bridge", "brief", "bright", "bring", "broad",
    "broken", "bronze", "brother", "brush", "bubble", "buddy", "budget", "buffalo", "build",
    "bulk", "bullet", "bundle", "burden", "burst", "butter", "buyer", "cabin", "cable",
    "cactus", "cage", "camera", "camp", "canal", "cancel", "candy", "canyon", "capable",
    "capital", "captain", "carbon", "cargo", "carpet", "carry", "castle", "casual", "catalog",
    "catch", "cattle", "ceiling", "celery", "cement", "census", "cereal", "certain", "chair",
    "chalk", "champion", "change", "chapter", "charge", "chase", "cheap", "cheese", "cherry",
    "chicken", "chief", "child", "chimney", "choice", "chunk", "cinema", "circle", "citizen",
    "claim", "clap", "clarify", "claw", "clay", "clean", "clerk", "clever", "click",
    "client", "cliff", "climb", "clock", "close", "cloth", "cloud", "clown", "cluster",
    "coach", "coconut", "code", "coffee", "coil", "collect", "color", "column", "combine",
    "comfort", "comic", "common", "company", "concert", "conduct", "confirm", "congress",
    "connect", "consider", "control", "convince", "cookie", "copper", "coral", "core",
    "corner", "correct", "couch", "country", "couple", "course", "cousin", "cover", "craft",
    "crash", "crater", "crazy", "cream", "credit", "crew", "cricket", "crime", "crisp",
    "critic", "crop", "cross", "crouch", "crowd", "crucial", "cruel", "cruise", "crystal",
    "cube", "culture", "cupboard", "curious", "current", "curve", "cycle", "daddy", "damage",
    "dance", "danger", "daring", "dawn", "debate", "debris", "decade", "decline", "decorate",
    "deep", "defense", "define", "defy", "degree", "delay", "deliver", "demand", "denial",
    "dentist", "deny", "depart", "depend", "deposit", "depth", "derive", "desert", "design",
    "desk", "detail", "detect", "develop", "device", "devote", "diagram", "diamond", "diary",
    "diesel", "differ", "digital", "dignity", "dilemma", "dinner", "dinosaur", "direct",
    "discover", "disease", "display", "distance", "divert", "doctor", "dolphin", "domain",
    "donkey", "donor", "door", "double", "dragon", "drama", "drastic", "dream", "drift",
    "drink", "drop", "drum", "dumb", "dune", "during", "dust", "dutch", "dwarf",
    "dynamic", "eager", "eagle", "early", "earth", "easily", "east", "echo", "ecology",
    "edge", "edit", "educate", "effort", "eight", "either", "elbow", "elder", "electric",
    "elegant", "element", "elephant", "elite", "emerge", "emotion", "employ", "empty",
    "enable", "endless", "endorse", "enemy", "energy", "engine", "enjoy", "enough", "enrich",
    "enroll", "ensure", "enter", "entire", "entry", "envelope", "episode", "equal", "equip",
    "erode", "erosion", "error", "escape", "essay", "estate", "eternal", "evening", "evidence",
    "evil", "evolve", "exact", "example", "excess", "exchange", "excite", "exclude", "excuse",
    "execute", "exile", "exist", "expand", "expect", "expire", "explain", "expose", "express",
    "extend", "extra", "eyebrow", "fabric", "face", "faculty", "faint", "faith", "false",
    "family", "famous", "fancy", "fantasy", "fashion", "fatal", "father", "fatigue", "fault",
    "favorite", "feature", "february", "federal", "fence", "festival", "fetch", "fever",
    "fiber", "fiction", "field", "figure", "filter", "final", "finger", "finish", "fire",
    "first", "fiscal", "fitness", "flag", "flame", "flash", "flavor", "flight", "flip",
    "float", "flock", "floor", "flower", "fluid", "flush", "focus", "follow", "force",
    "forest", "forget", "fork", "fortune", "forum", "fossil", "foster", "found", "fragile",
    "frame", "frequent", "fresh", "friend", "fringe", "frog", "front", "frozen", "fruit",
    "fuel", "funny", "furnace", "fury", "future", "gadget", "galaxy", "gallery", "game",
    "garage", "garden", "garlic", "garment", "gather", "gauge", "gaze", "general", "genius",
    "gentle", "genuine", "gesture", "ghost", "giant", "gift", "giggle", "ginger", "giraffe",
    "glad", "glance", "glare", "glass", "globe", "gloom", "glory", "glove", "glow",
    "glue", "goat", "goddess", "gold", "gorilla", "gospel", "gossip", "govern", "grace",
    "grain", "grant", "grape", "grass", "gravity", "great", "green", "grid", "grief",
    "grill", "grip", "grocery", "group", "grow", "guard", "guess", "guide", "guitar",
    "habit", "half", "hammer", "hamster", "hand", "happy", "harbor", "harvest", "hawk",
    "hazard", "health", "heart", "heavy", "hello", "helmet", "hero", "hidden", "high",
    "hint", "history", "hobby", "hockey", "holiday", "hollow", "honest", "honey", "hood",
    "horizon", "horror", "horse", "hospital", "hotel", "hover", "humble", "humor", "hundred",
    "hungry", "hurdle", "hurry", "hybrid", "ice", "icon", "idea", "identify", "idle",
    "ignore", "illegal", "image", "imitate", "immense", "immune", "impact", "impose",
    "improve", "impulse", "include", "income", "index", "indicate", "indoor", "infant",
    "inflict", "inform", "initial", "inject", "inner", "innocent", "input", "inquiry",
    "insane", "insect", "inside", "inspire", "install", "intact", "interest", "invest",
    "invite", "involve", "iron", "island", "isolate", "issue", "ivory", "jacket", "jaguar",
    "jewel", "join", "joke", "journey", "judge", "juice", "jump", "jungle", "junior",
    "junk", "just", "kangaroo", "keen", "kernel", "kick", "kidney", "kind", "kingdom",
    "kitchen", "kite", "kitten", "knife", "knock", "label", "labor", "ladder", "lady",
    "lake", "lamp", "language", "laptop", "large", "later", "latin", "laugh", "laundry",
    "layer", "leader", "leaf", "learn", "leave", "lecture", "left", "legend", "leisure",
    "lemon", "length", "lens", "letter", "level", "liberty", "library", "license", "life",
    "light", "likely", "limit", "link", "lion", "liquid", "list", "little", "live",
    "lizard", "lobster", "local", "lock", "logic", "lonely", "loop", "lottery", "loud",
    "lounge", "lumber", "lunar", "lunch", "luxury", "lyrics", "machine", "magic", "magnet",
    "major", "mammal", "manage", "mandate", "mango", "mansion", "manual", "maple", "marble",
    "march", "margin", "marine", "market", "master", "match", "material", "matter", "maximum",
    "meadow", "media", "melody", "member", "memory", "mentor", "mercy", "merge", "merit",
    "method", "middle", "million", "mimic", "minimum", "minor", "minute", "miracle", "mirror",
    "misery", "mistake", "mixture", "mobile", "model", "modify", "moment", "monitor", "monkey",
    "monster", "month", "moral", "morning", "mosquito", "mother", "motion", "motor",
    "mountain", "mouse", "movie", "much", "multiply", "muscle", "museum", "music", "must",
    "mutual", "myself", "mystery", "myth", "naive", "narrow", "nation", "nature", "near",
    "neck", "needle", "negative", "neglect", "neither", "nephew", "nerve", "nest", "network",
    "neutral", "never", "night", "noble", "noise", "normal", "north", "notable", "nothing",
    "notice", "novel", "nuclear", "number", "nurse", "object", "oblige", "observe", "obtain",
    "obvious", "occur", "ocean", "october", "odor", "office", "olive", "olympic", "omega",
    "onion", "online", "open", "opera", "opinion", "oppose", "option", "orange", "orbit",
    "orchard", "order", "organ", "orient", "original", "orphan", "ostrich", "other", "outdoor",
    "output", "outside", "oval", "oven", "owner", "oxygen", "oyster", "ozone", "palace",
    "panda", "panel", "panic", "panther", "paper", "parade", "parent", "park", "parrot",
    "patch", "patient", "patrol", "pattern", "pause", "peanut", "pepper", "perfect", "permit",
    "phrase", "picnic", "picture", "piece", "pilot", "pioneer", "pizza", "planet", "plastic",
    "plate", "pledge", "pluck", "plug", "plunge", "poem", "polar", "police", "pond",
    "popular", "portion", "position", "possible", "potato", "pottery", "poverty", "powder",
    "power", "practice", "predict", "prepare", "present", "pretty", "prevent", "price",
    "pride", "primary", "priority", "prison", "private", "problem", "process", "produce",
    "profit", "program", "project", "promote", "proof", "property", "prosper", "protect",
    "proud", "provide", "public", "pulse", "pumpkin", "punch", "pupil", "purple", "purpose",
    "puzzle", "pyramid", "quality", "quantum", "quarter", "question", "quick", "quote",
    "rabbit", "raccoon", "radar", "radio", "raise", "rally", "ranch", "random", "rapid",
    "rather", "razor", "ready", "reason", "rebel", "recipe", "record", "recycle", "reduce",
    "reflect", "reform", "region", "regret", "regular", "reject", "relax", "release", "relief",
    "remain", "remember", "remind", "remove", "render", "renew", "repair", "repeat", "replace",
    "report", "require", "rescue", "resist", "resource", "response", "result", "retire",
    "retreat", "return", "reveal", "review", "reward", "rhythm", "ribbon", "rice", "rich",
    "ride", "rifle", "rigid", "ring", "ripple", "rival", "river", "road", "robot",
    "robust", "rocket", "romance", "roof", "rookie", "room", "rough", "round", "route",
    "royal", "rubber", "rude", "rugby", "ruler", "rural", "saddle", "sadness", "safe",
    "sailor", "salad", "salmon", "salon", "sample", "sand", "satisfy", "sauce", "sausage",
    "save", "scale", "scene", "scheme", "school", "science", "scissors", "scorpion", "scout",
    "screen", "script", "search", "season", "second", "secret", "section", "security", "seed",
    "select", "senior", "sense", "sentence", "series", "service", "session", "settle", "setup",
    "seven", "shadow", "shaft", "shallow", "share", "shed", "shell", "sheriff", "shield",
    "shift", "shine", "ship", "shiver", "shock", "short", "shoulder", "shove", "shrimp",
    "shuffle", "sibling", "siege", "sight", "sign", "silent", "silver", "similar", "simple",
    "since", "siren", "sister", "situate", "skill", "skull", "slender", "slice", "slim",
    "slogan", "smart", "smile", "smoke", "smooth", "snake", "soccer", "social", "soldier",
    "solid", "solution", "someone", "song", "sort", "south", "space", "spare", "spatial",
    "spawn", "special", "speed", "sphere", "spider", "spirit", "split", "sponsor", "spoon",
    "sport", "spray", "spread", "spring", "square", "squeeze", "stable", "stadium", "staff",
    "stage", "stairs", "stamp", "stand", "start", "steak", "steel", "stem", "step",
    "stereo", "stick", "still", "stone", "stool", "story", "strategy", "street", "strike",
    "strong", "struggle", "student", "stuff", "stumble", "style", "subject", "submit",
    "subway", "success", "suffer", "sugar", "suggest", "summer", "sun", "supply", "supreme",
    "surface", "surge", "surprise", "surround", "survey", "suspect", "sustain", "swallow",
    "swamp", "sweet", "swim", "switch", "sword", "symbol", "symptom", "system", "table",
    "tackle", "talent", "target", "tank", "tape", "tattoo", "taxi", "teach", "team",
    "tell", "tenant", "tennis", "tent", "term", "test", "text", "theme", "theory",
    "therapy", "thing", "thought", "three", "thrive", "throw", "thumb", "thunder", "ticket",
    "tiger", "timber", "time", "tiny", "tired", "title", "toast", "tobacco", "today",
    "together", "token", "tomato", "tomorrow", "tone", "tongue", "tonight", "tool", "tooth",
    "topic", "torch", "tornado", "tortoise", "total", "tourist", "toward", "tower", "town",
    "trade", "traffic", "train", "transfer", "trash", "travel", "tray", "treat", "tree",
    "trend", "trial", "tribe", "trick", "trigger", "trim", "trophy", "trouble", "truck",
    "trumpet", "trust", "truth", "tumble", "tunnel", "turkey", "turtle", "twelve", "twenty",
    "twice", "twin", "twist", "umbrella", "unable", "unaware", "uncle", "under", "unfair",
    "unfold", "unhappy", "uniform", "unique", "unit", "universe", "unknown", "unlock",
    "unusual", "unveil", "update", "upgrade", "upper", "upset", "urban", "usage", "useful",
    "usual", "utility", "vacant", "vacuum", "valid", "valley", "valve", "vanish", "vapor",
    "various", "vault", "vehicle", "velvet", "vendor", "venture", "verify", "version",
    "vessel", "veteran", "viable", "vibrant", "victim", "victory", "video", "village",
    "vintage", "violin", "virtual", "virus", "visit", "visual", "vital", "vivid", "voice",
    "volcano", "volume", "voyage", "waffle", "wagon", "wallet", "walnut", "wander", "warfare",
    "warm", "warrior", "wash", "waste", "water", "wave", "wealth", "weapon", "weather",
    "wedding", "weekend", "weird", "welcome", "west", "whale", "wheat", "wheel", "whisper",
    "width", "wild", "window", "wine", "wing", "winner", "winter", "wisdom", "wise",
    "wish", "witness", "wolf", "woman", "wonder", "wood", "world", "worry", "worth",
    "wrap", "wreck", "wrestle", "wrist", "write", "wrong", "yard", "year", "yellow",
    "young", "youth", "zebra", "zero", "zone",
];

#[derive(Serialize, Deserialize)]
pub struct PasswordOptions {
    pub length: usize,
    pub count: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
    pub custom_symbols: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PassphraseOptions {
    pub word_count: usize,
    pub separator: String,
    pub capitalize: bool,
    pub include_number: bool,
}

#[derive(Serialize, Deserialize)]
pub struct StrengthResult {
    pub level: String,
    pub score: u8,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResult {
    pub passwords: Vec<String>,
    pub strength: StrengthResult,
}

fn build_charset(options: &PasswordOptions) -> String {
    let mut charset = String::new();
    if options.uppercase {
        charset.push_str(UPPERCASE);
    }
    if options.lowercase {
        charset.push_str(LOWERCASE);
    }
    if options.numbers {
        charset.push_str(DIGITS);
    }
    if options.symbols {
        if let Some(ref custom) = options.custom_symbols {
            charset.push_str(custom);
        } else {
            charset.push_str(SYMBOLS);
        }
    }
    if charset.is_empty() {
        charset.push_str(LOWERCASE);
    }
    charset
}

fn generate_single_password(charset: &str, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn calculate_entropy(pool_size: usize, length: usize) -> f64 {
    if pool_size <= 1 {
        return 0.0;
    }
    length as f64 * (pool_size as f64).log2()
}

fn evaluate_strength(entropy: f64) -> (String, u8) {
    if entropy < 30.0 {
        ("Too easy to guess!".to_string(), 1)
    } else if entropy < 50.0 {
        ("Not strong enough".to_string(), 2)
    } else if entropy < 70.0 {
        ("Getting there...".to_string(), 3)
    } else if entropy < 90.0 {
        ("Now we're talking!".to_string(), 4)
    } else {
        ("You're crazy!".to_string(), 5)
    }
}

#[tauri::command]
fn generate_passwords(options: PasswordOptions) -> GenerateResult {
    let charset = build_charset(&options);
    let pool_size = charset.chars().count();
    let entropy = calculate_entropy(pool_size, options.length);
    let (level, score) = evaluate_strength(entropy);

    let passwords: Vec<String> = (0..options.count)
        .map(|_| generate_single_password(&charset, options.length))
        .collect();

    GenerateResult {
        passwords,
        strength: StrengthResult { level, score },
    }
}

#[tauri::command]
fn generate_passphrase(options: PassphraseOptions) -> GenerateResult {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = (0..options.word_count)
        .map(|_| {
            let word = WORD_LIST[rng.gen_range(0..WORD_LIST.len())].to_string();
            if options.capitalize {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                }
            } else {
                word
            }
        })
        .collect();

    if options.include_number {
        let num: u16 = rng.gen_range(0..1000);
        let pos = rng.gen_range(0..=words.len());
        words.insert(pos, num.to_string());
    }

    let passphrase = words.join(&options.separator);
    let entropy = (options.word_count as f64) * (WORD_LIST.len() as f64).log2()
        + if options.include_number { 10.0 } else { 0.0 };
    let (level, score) = evaluate_strength(entropy);

    GenerateResult {
        passwords: vec![passphrase],
        strength: StrengthResult { level, score },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_passwords, generate_passphrase])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
