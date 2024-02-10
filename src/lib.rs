use scrypto::prelude::*;

#[blueprint]
mod ASTRLSTAKING {

    // Setting the access rules 
    enable_method_auth! { 
        methods { 

            add_stake => PUBLIC; 
            remove_stake => PUBLIC; 
            airdrop => restrict_to: [OWNER];
            
        }
    
    }

    struct ASTRLSTAKING {

        sastrl: Global<OneResourcePool>,

    }

    impl ASTRLSTAKING {
        pub fn new(owner_badge: ResourceAddress, ra: ResourceAddress) -> Global<OneResourcePool> {

            let (address_reservation, component_address) = Runtime::allocate_component_address(ASTRLSTAKING::blueprint_id());
            let global_component_caller_badge = NonFungibleGlobalId::global_caller_badge(component_address);

            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            let sastrl = Blueprint::<OneResourcePool>::instantiate(
                owner_role.clone(),
                rule!(require(global_component_caller_badge)), 
                ra,
                None
            );

            let mut component = Self {

                sastrl

            }

            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(
                rule!(require(owner_badge))
            ))
            .with_address(address_reservation)
            .globalize();

            return sastrl
        }

        pub fn remove_stake(&mut self, sastrl: Bucket) -> Bucket {

            let tokens = self.sastrl.redeem(sastrl);
            return tokens

        }

        pub fn add_stake(&mut self, astrl: Bucket) -> Bucket {

            let tokens = self.sastrl.contribute(astrl);
            return tokens

        }

        pub fn airdrop(&mut self, astrl: Bucket){

            self.sastrl.protected_deposit(astrl);
            return

        }

    }
}
