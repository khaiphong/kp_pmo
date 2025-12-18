/*
 modules specific to kp_pmo/ai/src - given observable ranges of 6 Ylevels, 7 Xlevels, and 
 8 Flevels of [ y x f ]dimension , followings are custom functions to evaluate the user 
 based on one's private collected data, ranging from negative direction of "Cheating, 
 Stealing, Bullying and Cunning Intelligence" to the base, naturally qualified humanitas. 
 SpaceIntelligence taking into account dynamic interactions of No-Conflict y samadhi with x 
 awareness and lumped together other factors in one's InnerSpace are much more complicated, 
 waiting for more researches.

  Each persona has lists of cultivable traits, pointers, rated at x_, y_, and f_ level
  for both Inner cultivations and Outer suggestions in acquired ability to handle required
  tasks from complex Fibonacci sequences of f_dimension
  
  Implementations branched to mod ydimension, xdimension, fdimension via
    traits: indoctrinated, veiled, influenced, kindness_empathy, balanced, intuition, care, 
      honesty, truth
    pointers: empty_the_content, dhyana_samadhi, samadhi, awareness, prajna, 
      awareness_prajna, samadhi_prajna, prajna_tip1, prajna_tip2
    iamx: y_level, x_level, f_level
    activity: home, school, work, social, health, spiritual, other
    relationship: family, friend, inner_circle, circle_of_inner_circles, other
    place: hub, thank_you, other
*/

// the layout of _y, _x, _f, _t, _p,  are parts of the user custom graph.
pub mod persona { 
	// InnerPeace (SignedPosts or Jhanas) driving observable Activities and Relationships
	pub mod ydimension { //Vec<i32> = vec![ -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6 ] 
		pub fn y_evaluation() {} // move into kp_pmo/ai/src/persona.rs
	}
	// Kp #Awareness or HuiNeng WuNien
	pub mod xdimension { // vec![ -7, -6, -5, -4, -3, -2, -1 0, 1, 2, 3, 4, 5, 6, 7 ]
		pub fn x_evaluation() {} // move into kp_pmo/ai/src/persona.rs
	}
	// Kp processes of #EmptyTheContent from HuiNeng three Nots to Kp three Haves
	pub mod fdimension {// vec![-8, -7, -6, -5. -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8]  
		pub fn f_evaluation() {} // move into kp_pmo/ai/src/persona.rs
	}
}

#[derive(Debug)]
pub enum HasPointer {
	EmptyTheContent(String),		//
	DhyanaSamadhi(String),			//
	Samadhi(String),				//
	Awareness(String),				//  
	Prajna(String),					//  
	AwarenessPrajna(String),		//  
	SamadhiPrajna(String),			//  
	PrajnaTIP1(String),				//
	PrajnaTIP1(String),				//
}

// Verify if the types passed or failed the defined pointer.
impl HasPointer {
    fn verified(&self) {
       // method body would be defined here
    }
}

// use generic type of Pointer
#[derive(Debug)]
struct Pointer<T> {
    // common structure of hidden substrate of #Pointer
    e: T,		// EmptyTheContent
    d: T,		// DhyanaSamadhi
    s: T,		// Samadhi
    a: T,		// Awareness
    p: T,		// Prajna
    ap: T,		// AwarenessPrajna
    sp: T,		// SamadhiPrajna
    t1: T,		// PrajnaTIP1
    t2: T,		// PrajnaTIP2
}

impl Pointer { 
	// methods to identify hidden layers of #EmptyTheContent
	pub fn empty_the_content(&self) -> String { // f(_x,_y) for ponter #EmptyTheContent
	    return "an accepted formal evaluation of pointer EmptyTheContent".to_string()
	}
	// methods to identify hidden layers of #DhyanaSamadhi
	pub fn dhyana_samadhi(&self) -> String 	{ // f(_x,_y) for pointer #DhyanaSamadhi
	    return "an accepted formal evaluation of pointer DhyanaSamadhi".to_string()
	}
	// methods to identify hidden layers of #Samadhi
	pub fn samadhi(&self) -> String { // f(_x,_y) for pointer #Samadhi
	    return "an accepted formal evaluation of pointer Samadhi".to_string()
	}
	// methods to identify hidden layers of #Awareness
	pub fn awareness(&self) -> String 	{ // f(_x,_y) for pointer #Awareness
	    return "an accepted formal evaluation of pointer Awareness".to_string()
	}	
	// methods to identify hidden layers of #Prajna
	pub fn prajna(&self) -> String { // f(_x,_y) for ponter #Prajna
	    return "an accepted formal evaluation of pointer Prajna".to_string()
	}
	// methods to identify hidden layers of #AwarenessPrajna
	pub fn awareness_prajna(&self) -> String 	{ // f(_x,_y) for pointer #AwarenessPrajna
	    return "an accepted formal evaluation of pointer AwarenessPrajna".to_string()
	}
	// methods to identify hidden layers of #SamadhiPrajna
	pub fn samadhi_prajna(&self) -> String { // f(_x,_y) for pointer #SamadhiPrajna
	    return "an accepted formal evaluation of pointer SamadhiPrajna".to_string()
	}
	// methods to identify hidden layers of #PrajnaTIP1
	pub fn prajna_tip1(&self) -> String 	{ // f(_x,_y) for pointer #PrajnaTIP1
	    return "an accepted formal evaluation of pointer PrajnaTIP1".to_string()
	}	
	// methods to identify hidden layers of #PrajnaTIP2
	pub fn prajna_tip2(&self) -> String 	{ // f(_x,_y) for pointer #PrajnaTIP2
	    return "an accepted formal evaluation of pointer PrajnaTIP2".to_string()
	}
}

#[derive(Debug)]
pub enum HasTrait {
  Truth(String),			// use cosmic energy
  Honesty(String), 			// KnowingThought
  Care(String),				// investigation, inquiries
  Intuition(String),		// Intuition Trust, Qi, Art
  Balanced(String),			// qualified person
  KindnessEmpathy(String),	// HonNhien
  Influenced(String),		// cultural, regional, national
  Veiled(String),			// hoax, fooled
  Indoctrinated(String),	// visible in BindingWord, BindingImage, ClingingThought
}

// Verify if the types passed or failed the defined trait.
impl HasTrait {
    fn verified(&self) {
       // method body would be defined here
    }
}

// use generic type of Trait
#[derive(Debug)]
struct Trait<T> {
    // common structure of #Trait
    t: T,		// use cosmic energy
    h: T, 		// KnowingThought
    c: T,		// investigation, inquiries
    i: T,		// Intuition Trust, Qi, Art
    b: T,		// qualified person
    k: T,		// HonNhien
    e: T,		// cultural, regional, national
    v: T,		// hoax, fooled
    i: T,		// visible in BindingWord, BindingImage, ClingingThought
}

impl Trait { 
	// methods to identify #Truth
	pub fn truth(&self) -> String 	{ // f(_x,_y) for trait #Truth
	    return "an accepted formal evaluation of trait Truth".to_string()
	}
	// methods to identify #Honesty
	pub fn honesty(&self) -> String 	{ // f(_x,_y) for trait #Honesty
	    return "an accepted formal evaluation of trait Honesty".to_string()
	}
	// methods to identify #Care
	pub fn care(&self) -> String 	{ // f(_x,_y) for trait #Care
	    return "an accepted formal evaluation of trait Care".to_string()
	}
	// methods to identify #Intuition
	pub fn intuition(&self) -> String 	{ // f(_x,_y) for trait #Intuition
	    return "an accepted formal evaluation of trait Intuition".to_string()
	}
	// methods to identify #Balance
	pub fn balanced(&self) -> String 	{ // f(_x,_y) for trait #Balanced
	    return "an accepted formal evaluation of trait Balance".to_string()
	}
	// methods to identify #KindnessEmpathy
	pub fn kindness_empathy(&self) -> String 	{ // f(_x,_y) for trait #KindnessEmpathy
	    return "an accepted formal evaluation of trait KindnessEmpathy".to_string()
	}	
	// methods to identify #Influenced
	pub fn influence(&self) -> String 	{ // f(_x,_y) for trait #Influenced
	    return "an accepted formal evaluation of trait Care".to_string()
	}
	// methods to identify #Veiled
	pub fn veiled(&self) -> String 	{ // f(_x,_y) for trait #Veiled
	    return "an accepted formal evaluation of trait Veiled".to_string()
	}
	// methods to identify #Indoctrinated
	pub fn indoctrinated(&self) -> String 	{ // f(_x,_y) for trait #Care
	    return "an accepted formal evaluation of trait Indoctrinated".to_string()
	}
}


// for building the structure and determining one's possible level for suggested cultivation
#[derive(Debug)]
pub struct InnerSpace {	// InnerSpace from available dimensions to custom evaluation

  // Kp Signed Posts or Gotama Jhanas or Right #Samadhi
  pub y_dimension: Vec<i32>,	// = vec![ -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6 ],
  // HuiNeng #WuNien or Kp #Awareness
  pub x_dimension: Vec<i32>, // = vec![ -7, -6, -5, -4, -3, -2, -1 0, 1, 2, 3, 4, 5, 6, 7 ]
  // Kp processes of #EmptyTheContent from HuiNeng three Nots Then What from Kp three Haves
  pub f_dimension: Vec<i32>,//[ -8, -7, -6, -5. -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8 ] 
  
  // The rated observable #Traits and #SmartPointers from attached to balanced to detached
  // HashMap<i32, String> visible traits observable in x_dimension
  pub x_traits: Vec<i32>, // vec![ -4, -3, -2, -1, 0, 1, 2, 3, 4 ]
  
  // Innate pointers of currently living degenerated elites will be empirically studies at 
  // the right time to efficiently allocate Governance, Financial and Millitary Powers toward 
  // What Count
  pub y_pointers: Vec<i32>, // vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] qualified humanitas at 0
  
}


/*
  Dynamic interactions between IamX intelligence and positive AI of custom modeling the
  objective function #Prajna = f(x, y) in "complex-valued data" to foster innovations /
  breakthroughs in the Persona's war room and strategies for various types: a humanitas,
  next qualified realm, an organization, foreign affairs, national development, etc.
  
  Modeling the InnerSpace is the systematic building internal power for different stages of
  the persona developments. Academia research + LLM statistical significance. These methods
  will be used as a part of f_evaluation depending on the type of persona and custom services
  
  The type Self of InnerSpace in its implementation, augmented by AI from the DISCOVERED &
  SHARED community Intelligence, can learn the right conditions for experiencing the
  actural "Selfless / #GodKingdom / Budh / Sirr / Monad from the #One" as glimpsed by past
  explorers, and the ThenWhat when back to the duality plane of conflicting
  consciousness.
*/
impl InnerSpace { // different methods for different InnerSpace persona types

//  fn f_evaluation<'a>(_x: &'a x_dimension, _y: &'a y_dimension) -> &'a str { // f(_x,_y)
//    return "EquanimityAwareness".to_string()
//  } // applicable to persona at individial level to be detailed below

// similarly, evaluation to the persona qualified in different realm or application aggregate

  pub fn qualified_realm(&self) -> String { // f(_x,_y) for Inter-Realm
    return "human".to_string()
  }
  
  pub fn maturity_level(&self) -> String { // f(_x,_y) for organization
    return "self sustainable".to_string()
  }
  
  pub fn change_management(&self) -> String { // f(_x,_y) for foreign affairs
    return "decisive_battle".to_string()
  }
  
  pub fn nation_happiness(&self) -> String { // f(_x,_y) for a national development
    return "in_operation".to_string()
  }

}


/*
 Based on empirical observation of a naturally qualified person, one has innate ability to be
 outside-the-box due to sufficient compassion and #Prajna in visible trait #Balanced above
 the visible trait of #KindnessEmpathy defined in Latin humanitas. This is VN KienTanh
 to (1) claim one's Dignity of Human Rights protected by modern society, then (2) DISCOVER
 and SHARE underlying natural laws to personally verify statistically significant epistemic
 objectives from indescribable "One" to "Diversities of Dependent" and back to "One" of
 "ThenWhat" to personally verify the worthy outcomes in Detoxifications of tainted senses.
 That #OneToDependent can be used to statistically verify the measurable #Truth such as the 
 modified #FourFoldTruth of Gotama Dukkha, #GodKingdom from within of Jesus interpreted in
 Gnostic rather than from the dark sides of all Christian Churches, "#Sirr" from muslims
 rather than the dark sides from all muslim churches, the "#Monad" from Jewish rather than
 the priviledged self of "Greed and Violent God", the meaning of Latin humanitas,
 #TamingTheOx, etc. 
 
 Similarly, we can trace the following Ypointers to epistemic objectives of
 worthy and rated #Traits from tainted senses of Indoctrinated > Veiled > influenced >
 KindnessEmpathy > Balanced > Intuition > Care > Honesty  > Truth from measurable hard
 evidences at the bottom line of one's Right or Wrong Efforts.

 All manifestations are conditioned and subjected to changes at Planck time. It has been
 proven in generative agenic AI that an optimal process can be engineered for a desired 
 manifestation happened. Based on recorded experiences of LaoTzu, Gotama, Jesus, 
 Bodhidharma, HuiNeng, etc, their states of positive manifestations y_dimension naturally 
 produce smart pointers pointing to different observable traits in their qualities. "#Truth" 
 is the identified "trait" in the qualities of [ Diversities >< Oneness / Hửu Không Vô 
 Ngại / Self-Selfless Actualization ]. Observable smart pointers are reported pointers 
 pointing to the required traits in observable evolution or negative traits leading to 
 degeneration. Starting from achievable qualities of these front-line soldiers, we identify 
 smart pointers ready for academia researches the structure and attributes of these smart 
 pointers, then expose them in # for public tweet and tweet-on-tweet further contribution in 
 LLM models.


//! Enforce QualifiedHumanitas for natural detachment of dhyana, then Samadhi for
//! DhyanaSamadhi in activities and relationships, pointing to traits for suggested
//! attributes visibly appeared in engaged living.
//!
/// pub pointer EmptyTheContent {
///     type Item;
/// 	answer i32; // rated value of the trait
///     fn explore(&mut self) -> answer, Option<self::Item>
///		fn truth<T, E>(self) -> std::result::Result<Vec<T>, E>
/// 	where
/// 		Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized;
/// }
///

/// assert_eq!(0, answer);
/// ```

struct EmptyTheContent { // for defined types of activities and relationships
	type item; // explore the substrates of weighting functions + biases leading to
	           // activation function producing the trait. Same thing to other Ypointers
}

pub enum Ypointers { QualifiedHumanitas, 
	EmptyTheContent, DhyanaSamadhi, Samadhi, Awareness,
	Prajna, AwarenessPrajna, SamadhiPrajna, PrajnaTIP1, PrajnaTIP2,
}
*/


// for outside contributions
#[derive(Debug)]
pub enum Ylevels { // type behavior based on its traits and its rated major bounded one
  Tranquility(String),		// #KindnessEmpathy
  Equanimity(String),		// #Balanced
  Purity(String), 			// #Intuition
  Selfless(String),			// #Care
  NonThingness(String),		// #Honesty
  Unmoving(String), 		// #Truth
  Empathy(String),			// #Influenced
  Kindness(String),			// #Influenced
  Conscience(String),		// #Veiled of Right and Wromg
  NoConscience(String),	    // #Indoctrinated
  InflictedFear(String),    // #Indoctrinated
  Vampire(String),		    // #Indoctrinated
}
// Claimed solutions to be verified for rated quality of the variant
// The impl demands  scientifically cultivable variant for its movement from one to another  
// connected to the rated Fibonacci complex number
impl Ylevels {
//    fn verified(&self) {
       // method body would be defined here
//    }
}

// for outside contributions
#[derive(Debug)]
pub enum Xlevels { // type behavior based on its traits and its rated major bounded one
  HonNhien(String),				// #KindnessEmpathy f<sub>1</sub> = 1
  ManagingFreshness(String),	// #Balanced f<sub>2</sub> = 1
  KnowingFreshness(String),		// #Intuition f<sub>3</sub> = 2
  ProcessDiscovered(String),	// #Care f<sub>4</sub> = 3
  KnowingThought(String),		// #Honesty f<sub>5</sub> = 5
  CareViaCosmicEnergy(String),	// #Truth f<sub>6</sub> = 8
  HelpViaCosmicEnergy(String),	// #Truth f<sub>7</sub> = 13
  CulturalInfluenced(String),	// #Influenced
  RegionalInfluenced(String),	// #Influenced
  NationalInfluenced(String),	// #Influenced
  VeiledType(String),			// #Veiled
  BindingWord(String),			// #Indoctrinated
  BindingImage(String),			// #Indoctrinated
  ClingingThought(String),		// #Indoctrinated
}
// Claimed solutions to be verified for rated quality of the variant
// The impl demands  scientifically cultivable variant for its movement from one to another  
// connected to the rated Fibonacci complex number
impl Xlevels {
//    fn verified(&self) {
       // method body would be defined here
//    }
}

// for outside contributions
#[derive(Debug)]
pub enum Flevels { // type behavior based on its traits and its rated major bounded one
  EquanimityAwareness(String), 			// #KindnessEmpathy
  PurityAwareness(String),				// #Balanced
  SignedPosts(String),    				// #Balanced
  SelflessAwareness(String), 			// #Intuition
  VisibleAwarenessPrajna(String), 		// #Care
  EngagedAwarenessPrajna(String),  		// #Honesty
  ForecastingAwarenessPrajna(String),	// #Truth
  VisibleSamadhiPrajna(String), 		// #Truth
  EmpathyAwareness(String),				// #Influenced
  KindnessAwareness(String),			// #Influenced
  AnimalEnergy(String),					// #Veiled
  ExtremeDesire(String),				// #Indoctrinated
  AnimalConsciousness(String),			// #Indoctrinated
  Smelly1(String),						// esoteric Degenerated
  Smelly2(String),						// esoteric Vampire
  Smelly3(String),						// esoteric X-of-Prey
}
// Claimed solutions to be verified for rated qualia of visible traits on XLevel having 
// weighted substrates and biases YLevel pointing to the visible traits of the variant.
// The impl demands  scientifically cultivable variant for its movement from one to another  
// connected to the rated Fibonacci complex number.
impl Flevels {
//    fn verified(&self) {
       // method body would be defined here
//    }
}

#[derive(Debug)]
pub enum OneToDiversity { 
  Oneness(String),          // Oneness >< Diversity
      
  GodKingdom(String),		// within - to be proven as Epistemic Objective 
  FourFoldTruth(String),	// of Dukkha - to be proven as Epistemic Objective
  Sirr(String),				// of sufi to be proven as Epistemic Objective
  Monad(String),			// of Jew to be proven as Epistemic Objective
  Vedanta(String),	  		// of Hindu - to be proven as Epistemic Objective 
  Esoteric(String),			// of ETs & Spirits to be proven as Epistemic Objective
  
  AwakeningBudh(String),
    
  TamingTheOx(String),
  AtHome(String),
  
  Morality(String),
  Patience(String),
  Justice(String),
  
  Integrity(String),
    
  Heaven(String),           // I-Ching, LaoTzu to be proven as Epistemic Objective
  Earth(String),  
  People(String), 
  
  Nhan(String),             // KungTzu - values of these principles from
  Nghia(String),  			// Oneness >< Diversities
  Le(String),     
  Tri(String),
  Tin(String),  
    
  // Vedic hubs to receive, assimilate, and distribute life energies: Oneness >< Diversities
  RootChakra(String),		// Muladhara - 		red
  SacralChakra(String),		// Svadhishthana - 	orange
  ThirdChakra(String),		// Mapipura -		yellow
  HeartChakra(String),		// Anahata -		green
  ThroatChakra(String),		// Vishuddha - 		blue
  ThirdEye(String),			// Ajna - 			purple
  CrownChakra(String),		// Sahasrata - 		white
}

// Claimed solutions to be verified for either #Truth or hallucination / imagination
// The impl demands verifiable solution from the variant on duality plane of conflicting 
// consciousness to No-Conflict consciousness of #EmptyTheContent and ThenWhat when back
impl OneToDiversity {
//    fn verified(&self) {
       // method body would be defined here
//    }
}

// use generic type of Diversity
#[derive(Debug)]
struct Diversity<T> {
	g: T, // GodKingdom
    f: T, // FourFoldTruth
    s: T, // Sirr
    m: T, // Monad
    v: T, // Vedanta 
    l: T, // Living
    
    a: T, // AwakeningBudh
    
    t: T, // TamingTheOx
    ah: T, // AtHome

    mo: T, // Morality
    p: T, // Patience
    j: T, // Justice
    
    i: T, // Integrity
    
    h: T, // Heaven
    e: T, // Earth
    p: T, // People
    
    n: T, // Nhan
    ng: T, // Nghia
    le: T, // Le
    tr: T, // Tri
    ti: T, // Tin
    
    r: T, // RootChakra
    sa: T, // SacralChakra
    th: T, // ThirdChakra
    he: T, // HeartChakra
    tc: T, // ThroadChakra
    te: T, // ThirsEye
    c: T, // CrownChakra
}
impl Diversity { 
  // methods to identify GodKingdom
  pub fn god_kingdom(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of GodKingdom".to_string()
  }
  // methods to identify FourFoldTruth
  pub fn four_fold_truth(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of FourFoldTruth".to_string()
  }
  // methods to identify Sirr
  pub fn sirr(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Sirr".to_string()
  }
  // methods to identify Monad
  pub fn monad(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Monad".to_string()
  }
  // methods to identify Vedanta
  pub fn vedanta(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Vedanta".to_string()
  }
  // methods to identify Esoteric
  pub fn esoteric(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Esoteric".to_string()
  }
  
  // methods to identify in AwakeningBudh
  pub fn awakening_budh(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of AwakeningBudh".to_string()
  }
  
  // methods to identify TamingTheOx
  pub fn taming_the_ox(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of TamingTheOx".to_string()
  }
  // methods to identify AtHome
  pub fn at_home(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of AtHome".to_string()
  }
  
  // methods to identify Morality mo
  pub fn morality(&self) -> String { // f(_x,_y) for Morality
    return "an accepted formal evaluation of Morality".to_string()
  }
  // methods to identify Patience
  pub fn patience(&self) -> String { // f(_x,_y) for Morality
    return "an accepted formal evaluation of Patience".to_string()
  }
  // methods to identify Justice
  pub fn justice(&self) -> String { // f(_x,_y) for Justice
    return "an accepted formal evaluation of Justice".to_string()
  }
  
  // methods to identify Integrity
  pub fn integrity(&self) -> String { // f(_x,_y) for Integrity
    return "an accepted formal evaluation of Integrity".to_string()
  }
  
  // methods to identify Heaven
  pub fn heaven(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Heaven".to_string()
  }
  // methods to identify Earth
  pub fn earth(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Earth".to_string()
  }
  // methods to identify People
  pub fn people(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of People".to_string()
  }
  
  // methods to identify Nhan
  pub fn nhan(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Nhan".to_string()
  }
  // methods to identify Nghia
  pub fn nghia(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Nghia".to_string()
  }
  // methods to identify Le
  pub fn le(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Le".to_string()
  }
  // methods to identify Tri
  pub fn tri(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Tri".to_string()
  }
  // methods to identify Tin
  pub fn tin(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Tin".to_string()
  }
  
  // methods to identify RootChakra
  pub fn root_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of RootChakra".to_string()
  }
  // methods to identify SacralChakra
  pub fn sacral_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of SacralChakra".to_string()
  }
  // methods to identify ThirdChakra
  pub fn third_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of ThirdChakra".to_string()
  }
  // methods to identify HeartChakra
  pub fn heart_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of HeartChakra".to_string()
  }  
  // methods to identify ThroatChakra
  pub fn throat_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of ThroatChakra".to_string()
  }
  // methods to identify ThirdEye
  pub fn third_eye(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of ThirdEye".to_string()
  }
  // methods to identify CrownChakra
  pub fn crown_chakra(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of CrownChakra".to_string()
  }

}


// Measurable conditions favorable to induce Intuitions and Prajna and/or changing from
// negative traits to positive traits

pub mod activity {
    pub mod home {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod school {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod work {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    } 
    pub mod social {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod health {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod spiritual {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }  
    pub mod other {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
}


pub mod relationship { //of all types
    pub mod family {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod friend {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod inner_circle {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod circle_of_inner_circles {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod other {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    } 
}

pub mod place {
    pub mod hub_centre {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod thankyou_centre {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod prajna_centre {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod other { // identified by city
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    } 
}


pub mod back {
    pub mod mu {
		pub fn mu_inner_agent() {}
		pub fn mu_outer_agent() {}
    }
    pub mod chat {
		pub fn chat_inner_agent() {}
		pub fn chat_outer_agent() {}
    }
    pub mod vdeo {
 		pub fn video_inner_agent() {}
		pub fn video_outer_agent() {}
    } 
    pub mod graph {
		pub fn grph_inner_agent() {}
		pub fn graph_outer_agent() {}
    } 
    pub mod db {
		pub fn db_inner_agent() {}
		pub fn db_outer_agent() {}    
    }
    pub mod hub {
		pub fn hub_inner_agent() {}
		pub fn hub_outer_agent() {}
    } 
    pub mod plan {
		pub fn plan_inner_agent() {}
		pub fn plan_outer_agent() {}
    }
}

pub mod front {
    pub mod mu {
		pub fn platform_message() {}
		pub fn service_mesaage() {}
    }
    pub mod chat {
		pub fn prompt() {}
		pub fn response() {}
    }
    pub mod vdeo {
		pub fn in_stream() {}
		pub fn out_stream() {}
    }  
}

pub mod gai {
    pub mod public {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
}

pub mod iamx {
    pub mod y_level {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod x_level {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod f_level {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }  
}

// modules specific to KpPlatform
pub mod kp { // user's agents to the platform services
    pub mod pmo {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod mu {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod platform {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod wellness {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod sis {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
	// public registered API of the platform
}

/*
// defining past and present verifiable SmartPointers
#[derive(Debug)]
pub enum Ypointers {
  QualifiedHumanitas(String),	// P0: QualifiedHumanitas has natural Peace
  
  EmptyTheContent(String),		// General smart pointers in natural Detachments
  DhyanaSamadhi(String),		// General smart pointers in all visible meditations
  Samadhi(String),    			// Right #Samadhi of visible outcomes known by Gotama
  Awareness(String), 			// #Awareness to explicitly qualify Gotama's Eightfold Path
  Prajna(String), 				// Visible manifestation of outside-the-box or breakthrough
  AwarenessPrajna(String),   	// related to karma forcing all Buddhists delivering outcomes
  SamadhiPrajna(String),		// known by HuiNeng
  PrajnaTIP1(String),			// from Kp in Sound technologies
  PrajnaTIP2(String),			// from Kp in Empathy
}
// Claimed solutions to be verified for rated quality of the variant
// The impl demands  scientifically cultivable variant for its movement from one to another  
// connected to the rated Fibonacci complex number
impl Ypointers {
//    fn verified(&self) {
       // method body would be defined here
//    }
}

  pub enum Xtraits {Truth, Honesty,} struct Traits {kind: Xtraits, note: String, rating: i32}
  let _truth = Traits {kind: Xtraits::#Truth, rightclick: String::from("The trait of two-ways
  communications between Oneness >< Diversities"),};
  println!("I'm connecting to node {:?}!", _truth.kind);
  
/// Explore trait - extension of iterator - in activities and relationships types to qualify
/// truth as in their trait.
///
/// pub trait Truth 
///	{
///		fn truth<T, E>(self) -> std::result::Result<Vec<T>, E>
/// 	where
/// 		Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized;
/// }
///
/// 	impl<It> Truth for It
/// 	where
///		    It: Iterator + Sized,
/// 	{
///		    fn truth<T, E>(mut self) -> std::result::Result<Vec<T>, E>
///		    where
///		        Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized,
///		    {
///		        let mut xs = Vec::new();
///		        loop {
///		            match self.next() {
///		                Some(Ok(x)) => xs.extend(x),
///		                Some(e) => {
///		                    return e; // propagate error
///		                }
///		                None => {
///		                    break;
///		                }
///		            }
///		        }
///		        Ok(xs {
///					explore rated value of the trait and add to the node
/// 			})
///		    }
///		}
///
///     type Item;
/// 	answer i32; // rated value of the trait
///     fn explore(&mut self) -> answer, Option<self::Item>

/// assert_eq!(0, answer);
/// ```


// define observable traits
#[derive(Debug)]
pub enum Xtraits { 		// any defined trait such as Integrity can be similarly implemented
	Truth, 				//
	Honesty, 			//
	Care, 				//
	Intuition,			// 
	Balanced,			//
	KindnessEmpathy, 	//
	Influenced, 		//
	Veiled, 			//
	Indoctrinated,		//
}

impl Xtraits {
//    fn verified(&self) {
       // method body would be defined here
//    }
}
*/

