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

// for each legal user, the layout of _y, _x, _f, _t, _p, _p1 -- _p9 are private and part of
// the user custom graph.
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


