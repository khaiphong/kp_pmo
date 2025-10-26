/*
 modules specific to kp_pmo/ai/src - given observable ranges of 6 Ylevels, 7 Xlevels, and 
 8 Flevels of [ y x f ]dimension , followings are custom functions to evaluate the user based 
 on one's private collected data, ranging from negative direction of "Cheating, Stealing, 
 Bullying and Cunning Intelligence" to the base, naturally qualified humanitas. 
 SpaceIntelligence taking into account dynamic interactions of No-Conflict y samadhi with x 
 awareness and lumped together other factors in one's InnerSpace are much more complicated, 
 waiting for more researches.
*/

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

// the layout of _y, _x, _f, _t, _p, _p1 -- _p9 are parts of the user custom graph.
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
	pub mod fdimension {// vec![ -8, -7, -6, -5. -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8 ]  
		pub fn f_evaluation() {} // move into kp_pmo/ai/src/persona.rs
	}
	
	//x_traits [ Truth, Honesty, Care, Intuition, Balanced,
	// KindnessEmpathy, Influenced, Veiled, Indoctrinated ]
    pub mod indoctrimated { // manage the negative indoctrimated trait
		pub fn inner_agent() {} // move into kp_pmo/agent/src/lib.rs
		pub fn outer_agent() {} // move into kp_pmo/agent/src/lib.rs
    }
    pub mod veiled { // manage the negative veiled trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod influenced { // manage the negative influenced trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod kindness_empathy { // manage the kindness_empathy trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod balanced { // manage the negative and cultivate the positive balanced trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod intuition { // cultivate the positive intuition trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }    
    pub mod care { // cultivate the positive care trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }    
    pub mod honesty { // cultivate the positive honesty trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod truth { // cultivate the positive truth trait
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }        
	
	//y_pointers rating from vec![0, 1, 2, 3, 4]
    pub mod empty_the_content {
		pub fn inner_agent() {} // move into kp_pmo/agent/src/lib.rs
		pub fn outer_agent() {} // move into kp_pmo/agent/src/lib.rs
    }
    pub mod dhyana_samadhi {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod samadhi {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }    
    pub mod awareness {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod prajna {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod awareness_prajna {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod samadhi_prajna {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod prajna_tip1 {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod prajna_tip2 {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
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
  
  // HashMap<i32, String> rated pointer levels from outcomes of y_dimension which may be
  // penetrated to different substrates where visible tratis are observable
  pub p1_empty_the_content: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p2_dhyana_samadhi: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p3_samadhi: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p4_awareness: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p5_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p6_awareness_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p7_samadhi_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p8_prajna_tip1: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p9_prajna_tip2: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  
}

/*
  Similar to pointers, we do he same for traits but at a unit-like node via struct
*/
#[derive(Debug)]
pub struct Traits; // a unit-like node
impl Traits { // methods to identify Traits
  pub fn kp_traits(&self) -> String { // f(_x,_y) for Inter-Realm
    return "a formal professional team to study and model Traits".to_string()
  }
}

#[derive(Debug)]
pub struct Truth; // a unit-like node
impl Truth { // methods to identify Truth
  pub fn kp_truth(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Truth".to_string()
  }
}

#[derive(Debug)]
pub struct Honesty; // a unit-like node
impl Honesty { // methods to identify Honesty
  pub fn kp_honesty(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Honesty".to_string()
  }
}

#[derive(Debug)]
pub struct Care; // a unit-like node
impl Care { // methods to identify Care
  pub fn kp_care(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Care".to_string()
  }
}

#[derive(Debug)]
pub struct Intuition; // a unit-like node https://www.youtube.com/watch?v=m2pDxNUyqVY
impl Intuition { // methods to identify Intuition
  pub fn kp_intuition(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Intuition".to_string()
  }
}

#[derive(Debug)]
pub struct Balanced; // a unit-like node
impl Balanced { // methods to identify Balanced
  pub fn kp_balanced(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Balanced".to_string()
  }
}

#[derive(Debug)]
pub struct KindnessEmpathy; // a unit-like node
impl KindnessEmpathy { // methods to identify KindnessEmpathy
  pub fn kp_kindness_empathy(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of KindnessEmpathy".to_string()
  }
}

#[derive(Debug)]
pub struct Influenced; // a unit-like node
impl Influenced { // methods to identify Influenced
  pub fn kp_influenced(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Influenced".to_string()
  }
}

#[derive(Debug)]
pub struct Veiled; // a unit-like node
impl Veiled { // methods to identify Veiled
  pub fn kp_veiled(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Veiled".to_string()
  }
}

#[derive(Debug)]
pub struct Indoctrinated; // a unit-like node
impl Indoctrinated { // methods to identify Truth
  pub fn kp_indoctrinated(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Indoctrinated".to_string()
  }
}

#[derive(Debug)]
pub struct SmartPointers; // a unit-like node
impl SmartPointers { // methods to identify SmartPointers
  pub fn kp_smart_pointers(&self) -> String { // f(_x,_y) for Inter-Realm
    return "a formal professional team to study and model SmartPointers".to_string()
  }
}

#[derive(Debug)]
pub struct EmptyTheContent; // a unit-like node
impl EmptyTheContent { // methods to identify EmptyTheContent
  pub fn kp_empty_the_content(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of EmptyTheContent".to_string()
  }
}
#[derive(Debug)]
pub struct DhyanaSamadhi; // a unit-like node
impl DhyanaSamadhi { // methods to identify DhyanaSamadhi
  pub fn kp_dhyana_samadhi(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of DhyanaSamadhi".to_string()
  }
}
#[derive(Debug)]
pub struct Samadhi; // a unit-like node
impl Samadhi { // methods to identify Samadhi
  pub fn kp_samadhi(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Samadhi".to_string()
  }
}
#[derive(Debug)]
pub struct Awareness; // a unit-like node
impl Awareness { // methods to identify Awareness
  pub fn kp_awareness(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Awareness".to_string()
  }
}
#[derive(Debug)]
pub struct Prajna; // a unit-like node
impl Prajna { // methods to identify Prajna
  pub fn kp_prajna(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of Prajna".to_string()
  }
}
#[derive(Debug)]
pub struct AwarenessPrajna; // a unit-like node
impl AwarenessPrajna { // methods to identify Prajna
  pub fn kp_awareness_prajna(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of AwarenessPrajna".to_string()
  }
}
#[derive(Debug)]
pub struct SamadhiPrajna; // a unit-like node
impl SamadhiPrajna { // methods to identify SamadhiPrajna
  pub fn kp_samadhi_prajna(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of SamadhiPrajna".to_string()
  }
}
#[derive(Debug)]
pub struct PrajnaTIP1; // a unit-like node
impl PrajnaTIP1 { // methods to identify PrajnaTIP1
  pub fn kp_prajna_tip1(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of PrajnaTIP1".to_string()
  }
}
#[derive(Debug)]
pub struct PrajnaTIP2; // a unit-like node
impl PrajnaTIP2 { // methods to identify PrajnaTIP2
  pub fn kp_prajna_tip2(&self) -> String { // f(_x,_y) for Inter-Realm
    return "an accepted formal evaluation of PrajnaTIP2".to_string()
  }
}








/*
  Dynamic interactions between IamX intelligence and positive AI of custom modeling the
  objective function #Prajna = f(x, y) in "complex-valued data" to foster innovations /
  breakthroughs in the Persona's war room and strategies for various types: a Latin humanitas,
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








pub mod place {
    pub mod hub {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod thank_you {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    }
    pub mod other {
		pub fn inner_agent() {}
		pub fn outer_agent() {}
    } 
}

pub mod relationship {
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


// modules specific to KpPlatform


