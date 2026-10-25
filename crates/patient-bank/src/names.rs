//! Fictional name pools for the synthetic panel, keyed by background and era.
//!
//! Any resemblance to a real person is coincidental; full names are made unique
//! within the panel by `design`. The tables keep the order of `names.py`, since
//! the panel draws from them by index.

pub type Names = &'static [&'static str];

#[rustfmt::skip]
pub const GIVEN: &[(&str, &str, Names)] = &[
    ("female", "older", &[
        "Barbara", "Patricia", "Carol", "Judith", "Joan", "Shirley", "Dorothy", "Marilyn",
        "Beverly", "Janet", "Elaine", "Lorraine", "Gloria", "Rita", "Phyllis", "Doris",
        "Margaret", "Eleanor", "Frances", "Ruth", "Arlene", "Constance", "Norma", "Lois",
        "Theresa", "Geraldine", "Rosemary", "Irene", "Bernice", "Jean", "Kathleen", "Diane",
        "Sandra", "Linda", "Nancy", "Sharon", "Carolyn", "Marjorie", "Evelyn", "Anita",
    ]),
    ("female", "middle", &[
        "Karen", "Lisa", "Michelle", "Kimberly", "Donna", "Laura", "Susan", "Denise", "Tammy",
        "Wendy", "Julie", "Christine", "Dawn", "Lori", "Tracy", "Heather", "Stephanie",
        "Angela", "Melissa", "Amy", "Jennifer", "Kelly", "Rebecca", "Andrea", "Monica", "Erin",
        "Shannon", "Holly", "Nicole", "Colleen", "Gretchen", "Beth", "Paula", "Jill", "Renee",
        "Sheila", "Valerie", "Maureen", "Sonya", "Robin",
    ]),
    ("female", "younger", &[
        "Emily", "Hannah", "Madison", "Abigail", "Olivia", "Sophia", "Chloe", "Grace", "Lauren",
        "Megan", "Rachel", "Samantha", "Alyssa", "Brianna", "Kayla", "Jasmine", "Taylor",
        "Morgan", "Paige", "Caitlin", "Natalie", "Zoe", "Lily", "Ava", "Isabelle", "Hailey",
        "Delaney", "Riley", "Maya", "Nora", "Clara", "Leah", "Ella", "Sadie", "Willa", "Iris",
        "Phoebe", "Tessa", "Juniper", "Margo",
    ]),
    ("male", "older", &[
        "Robert", "Richard", "Donald", "Gerald", "Kenneth", "Raymond", "Ronald", "Walter",
        "Harold", "Eugene", "Ralph", "Roger", "Arthur", "Norman", "Francis", "Leonard",
        "Edward", "Stanley", "Carl", "Bernard", "Frederick", "Lawrence", "Wayne", "Howard",
        "Russell", "Dennis", "Gordon", "Clifford", "Lloyd", "Herbert", "George", "Paul",
        "Thomas", "James", "Charles", "William", "Joseph", "Peter", "Alan", "Vernon",
    ]),
    ("male", "middle", &[
        "Michael", "David", "Mark", "Scott", "Jeffrey", "Brian", "Kevin", "Timothy", "Gregory",
        "Todd", "Craig", "Eric", "Steven", "Christopher", "Matthew", "Jason", "Shawn", "Troy",
        "Darren", "Keith", "Derek", "Brett", "Andrew", "Patrick", "Sean", "Jonathan", "Adam",
        "Glenn", "Neil", "Marc", "Kurt", "Joel", "Dale", "Chad", "Rodney", "Bryan", "Douglas",
        "Philip", "Randall", "Stuart",
    ]),
    ("male", "younger", &[
        "Tyler", "Ethan", "Noah", "Logan", "Jacob", "Ryan", "Dylan", "Connor", "Austin",
        "Brandon", "Kyle", "Zachary", "Cameron", "Evan", "Nathan", "Owen", "Caleb", "Hunter",
        "Mason", "Liam", "Lucas", "Gavin", "Colin", "Jack", "Wyatt", "Eli", "Henry", "Miles",
        "Theo", "Declan", "Silas", "Asher", "Rowan", "Finn", "Ezra", "August", "Calvin",
        "Simon", "Graham", "Malcolm",
    ]),
];

#[rustfmt::skip]
pub const GIVEN_HERITAGE: &[(&str, Names, Names)] = &[
    ("hispanic",
        &[
            "Maria", "Carmen", "Rosa", "Luz", "Ana", "Gladys", "Marisol", "Yolanda", "Daisy",
            "Jessenia", "Yesenia", "Lizbeth", "Iris", "Nilda", "Migdalia", "Sonia", "Brenda",
            "Valeria", "Camila", "Gabriela", "Natalia", "Adriana", "Lourdes", "Wanda", "Zoraida",
        ],
        &[
            "Jose", "Luis", "Carlos", "Angel", "Miguel", "Juan", "Hector", "Edwin", "Wilfredo",
            "Raul", "Ramon", "Javier", "Jorge", "Victor", "Rafael", "Orlando", "Efrain",
            "Julio", "Mateo", "Diego", "Adrian", "Christian", "Josue", "Elvin", "Nelson",
        ]),
    ("asian",
        &[
            "Mei", "Linh", "Thao", "Hana", "Ji-woo", "Priya", "Anjali", "Sokha", "Mai", "Yuki",
            "Huong", "Su-jin", "Lan", "Kavya", "Sreymom", "Xiaoling", "Nisha", "Hyun-joo",
            "Aiko", "Trang",
        ],
        &[
            "Wei", "Minh", "Tuan", "Hoang", "Jin", "Raj", "Arjun", "Sovann", "Kenji", "Hiroshi",
            "Duc", "Sung-min", "Bao", "Vikram", "Chen", "Dara", "Rohan", "Jae-won", "Long",
            "Anil",
        ]),
    ("black",
        &[
            "Denise", "Keisha", "Tamika", "Aisha", "Monique", "Latoya", "Imani", "Ebony", "Nia",
            "Jada", "Simone", "Yvette", "Brianna", "Danielle", "Tiana", "Gwendolyn", "Fatou",
            "Abena",
        ],
        &[
            "Marcus", "Darnell", "Andre", "Jamal", "Terrence", "Malik", "Kwame", "Tyrone",
            "Jerome", "Isaiah", "Elijah", "Cedric", "Desmond", "Lamar", "Reginald", "Kofi",
            "Ousmane", "Curtis",
        ]),
    ("slavic",
        &[
            "Olga", "Natalia", "Svetlana", "Irina", "Agnieszka", "Katarzyna", "Halina", "Oksana",
        ],
        &[
            "Sergei", "Dmitri", "Andrzej", "Piotr", "Viktor", "Yuri", "Tomasz", "Mykola",
        ]),
];

#[rustfmt::skip]
pub const FAMILY: &[(&str, Names)] = &[
    ("general", &[
        "Abbott", "Allard", "Archambault", "Bailey", "Baker", "Barrett", "Beaulieu", "Bennett",
        "Bergeron", "Bishop", "Blanchard", "Boucher", "Bowen", "Boyle", "Bradley", "Brennan",
        "Brooks", "Burke", "Callahan", "Carlson", "Carroll", "Carter", "Casey", "Chapman",
        "Clark", "Cole", "Collins", "Connolly", "Cote", "Coughlin", "Crowley", "Cummings",
        "Curran", "Daley", "Davidson", "Dean", "Delaney", "Desrosiers", "Dion", "Dolan",
        "Donovan", "Dowd", "Doyle", "Dubois", "Duffy", "Dunn", "Dupuis", "Ellis", "Emerson",
        "Farrell", "Fenton", "Finn", "Fitzgerald", "Flaherty", "Fleming", "Foley", "Fontaine",
        "Ford", "Foster", "Fournier", "Gagnon", "Gallagher", "Garvey", "Gauthier", "Gillespie",
        "Gilmore", "Goodwin", "Grady", "Graves", "Griffin", "Hale", "Hall", "Hanlon", "Hartley",
        "Hastings", "Hayward", "Healey", "Hebert", "Hennessey", "Hickey", "Holt", "Howland",
        "Hurley", "Ingalls", "Jacobs", "Jenkins", "Kane", "Keane", "Keegan", "Kelleher",
        "Kendall", "Kenney", "Kerrigan", "Kimball", "Kingsley", "Kirby", "Labrie", "Lacroix",
        "Lambert", "Landry", "Langevin", "Larkin", "Lavoie", "Leary", "Leblanc", "Lemieux",
        "Levesque", "Lynch", "Madden", "Maguire", "Mahoney", "Malloy", "Marsh", "Martel",
        "Mayhew", "McCarthy", "McDermott", "McGrath", "McKenna", "Mercier", "Merrill",
        "Michaud", "Moran", "Morin", "Mulcahy", "Murphy", "Nadeau", "Nolan", "Norris",
        "O'Brien", "O'Connell", "O'Leary", "Ouellette", "Paquette", "Parent", "Parsons",
        "Pelletier", "Perkins", "Pickett", "Plante", "Poirier", "Pratt", "Quinn", "Reardon",
        "Regan", "Richard", "Riley", "Roberge", "Rousseau", "Roy", "Ryan", "Sanborn", "Sargent",
        "Savoie", "Shea", "Sheehan", "Simard", "Slattery", "Soucy", "Sullivan", "Sweeney",
        "Talbot", "Tierney", "Tremblay", "Turcotte", "Vaillancourt", "Walsh", "Warner",
        "Webster", "Wheeler", "Whitcomb", "Wilder", "Winslow", "Woodward", "Wright", "Yeager",
        "Zielinski", "Kowalski", "Nowak", "Wojcik", "Kaminski", "Lewandowski", "Szymanski",
        "Dabrowski", "Mazur", "Rossi", "Russo", "Esposito", "Bianchi", "Romano", "Costa",
        "Ferraro", "Marino", "Greco", "Bruno", "Gallo", "Conti", "Santos", "Medeiros", "Silva",
        "Pereira", "Rezendes", "Sousa", "Andrade", "Cabral", "Goldberg", "Weiss", "Friedman",
        "Katz", "Levine", "Rosen", "Shapiro", "Stein", "Klein", "Berman", "Lindqvist",
        "Johansson", "Novak", "Horvath", "Schmidt", "Becker", "Hoffman", "Krause", "Wagner",
        "Fischer", "Nakamura", "Morrow", "Pierce", "Sawyer", "Thayer", "Upton", "Vose",
        "Whitney", "Alden", "Babcock", "Chandler", "Coolidge", "Damon", "Eldridge", "Fairbanks",
        "Gould", "Hatch", "Lyman", "Pomeroy", "Sears", "Strong", "Warren", "Wells", "Kearney",
        "Cronin", "Doherty", "Galvin",
    ]),
    ("hispanic", &[
        "Rivera", "Rodriguez", "Martinez", "Gonzalez", "Hernandez", "Lopez", "Perez", "Ortiz",
        "Cruz", "Torres", "Ramos", "Colon", "Santiago", "Vega", "Rosario", "Figueroa",
        "Morales", "Diaz", "Velazquez", "Rios", "Melendez", "Quiles", "Negron", "Burgos",
        "Matos", "Pagan", "Soto", "Acevedo", "Feliciano", "Alicea", "Castro", "Mendez", "Reyes",
        "Serrano",
    ]),
    ("asian", &[
        "Nguyen", "Tran", "Pham", "Le", "Chen", "Wang", "Li", "Zhang", "Liu", "Kim", "Park",
        "Lee", "Choi", "Patel", "Shah", "Sharma", "Gupta", "Singh", "Chhim", "Sok", "Pich",
        "Tanaka", "Sato", "Huang", "Lin", "Wu", "Hoang", "Vu", "Do", "Iyer", "Rao", "Jung",
        "Kang", "Ly",
    ]),
    ("black", &[
        "Johnson", "Williams", "Jackson", "Robinson", "Harris", "Thompson", "Washington",
        "Freeman", "Coleman", "Simmons", "Banks", "Mensah", "Owusu", "Diallo", "Toussaint",
        "Pierre", "Jean-Baptiste", "Grant", "Hughes", "Gaines", "Fields", "Reed", "Palmer",
        "Bryant",
    ]),
    ("slavic", &[
        "Ivanova", "Petrov", "Smirnov", "Kuznetsov", "Volkov", "Kowalczyk", "Wisniewski",
        "Shevchenko",
    ]),
];

#[rustfmt::skip]
pub const AREA_CODES: Names = &[
    "413", "774", "508", "617", "860", "802", "518", "978", "781", "203", "603", "401", "207",
    "347", "914",
];

/// Asian names are paired by origin so given and family names agree:
/// (origin, female given, male given, family).
#[rustfmt::skip]
pub const ASIAN_ORIGINS: &[(&str, Names, Names, Names)] = &[
    ("vietnamese",
        &[
            "Linh", "Thao", "Huong", "Trang", "Lan", "Mai", "Ngoc", "Phuong",
        ],
        &[
            "Minh", "Tuan", "Hoang", "Duc", "Bao", "Long", "Quang", "Thanh",
        ],
        &[
            "Nguyen", "Tran", "Pham", "Le", "Hoang", "Vu", "Do", "Ly",
        ]),
    ("chinese",
        &[
            "Mei", "Xiaoling", "Jing", "Yan", "Hui", "Lin",
        ],
        &[
            "Wei", "Chen", "Jun", "Hao", "Ming", "Lei",
        ],
        &[
            "Chen", "Wang", "Li", "Zhang", "Liu", "Huang", "Lin", "Wu",
        ]),
    ("korean",
        &[
            "Ji-woo", "Su-jin", "Hyun-joo", "Eun-ji", "Min-seo",
        ],
        &[
            "Jin", "Sung-min", "Jae-won", "Dong-hyun", "Min-jun",
        ],
        &[
            "Kim", "Park", "Lee", "Choi", "Jung", "Kang",
        ]),
    ("south_asian",
        &[
            "Priya", "Anjali", "Kavya", "Nisha", "Deepa", "Sunita",
        ],
        &[
            "Raj", "Arjun", "Vikram", "Rohan", "Anil", "Sanjay",
        ],
        &[
            "Patel", "Shah", "Sharma", "Gupta", "Singh", "Iyer", "Rao",
        ]),
    ("khmer",
        &[
            "Sokha", "Sreymom", "Chanthy", "Bopha", "Sophea",
        ],
        &[
            "Sovann", "Dara", "Vuthy", "Rithy", "Sokhom",
        ],
        &[
            "Chhim", "Sok", "Pich", "Chea", "Keo", "Men",
        ]),
    ("japanese",
        &[
            "Yuki", "Aiko", "Haruka", "Emi",
        ],
        &[
            "Kenji", "Hiroshi", "Takeshi", "Daisuke",
        ],
        &[
            "Tanaka", "Sato", "Nakamura", "Suzuki", "Watanabe",
        ]),
];

pub const LANGUAGE_ORIGIN: &[(&str, &str)] = &[
    ("Vietnamese", "vietnamese"),
    ("Mandarin", "chinese"),
    ("Korean", "korean"),
    ("Khmer", "khmer"),
    ("Hindi", "south_asian"),
];

fn find<T: Copy>(table: &[(&str, T)], key: &str) -> Option<T> {
    table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// `GIVEN[(sex, era)]`.
pub fn given(sex: &str, era: &str) -> Names {
    GIVEN
        .iter()
        .find(|(s, e, _)| *s == sex && *e == era)
        .map(|(_, _, n)| *n)
        .unwrap_or_else(|| panic!("no given names for ({sex}, {era})"))
}

/// `GIVEN_HERITAGE[heritage][sex]`.
pub fn given_heritage(heritage: &str, sex: &str) -> Names {
    let (_, female, male) = GIVEN_HERITAGE
        .iter()
        .find(|(h, _, _)| *h == heritage)
        .unwrap_or_else(|| panic!("no given names for {heritage}"));
    match sex {
        "female" => female,
        "male" => male,
        _ => panic!("no given names for sex {sex}"),
    }
}

/// `FAMILY[heritage]`.
pub fn family(heritage: &str) -> Names {
    find(FAMILY, heritage).unwrap_or_else(|| panic!("no family names for {heritage}"))
}

/// `ASIAN_ORIGINS[origin]`: (female given, male given, family).
pub fn asian_origin(origin: &str) -> (Names, Names, Names) {
    ASIAN_ORIGINS
        .iter()
        .find(|(o, ..)| *o == origin)
        .map(|(_, f, m, fam)| (*f, *m, *fam))
        .unwrap_or_else(|| panic!("no names for origin {origin}"))
}

/// `LANGUAGE_ORIGIN.get(language)`.
pub fn language_origin(language: &str) -> Option<&'static str> {
    find(LANGUAGE_ORIGIN, language)
}
