use clap::Parser;
use frost_dalek::keygen::Coefficients;
use frost_dalek::keygen::RoundOne;
use frost_dalek::DistributedKeyGeneration;
use frost_dalek::GroupKey;
use frost_dalek::keygen::SecretKey;
use log::warn;
use log::info;
use frost_dalek::Parameters;
use frost_dalek::Participant;

#[derive(Parser)]
#[derive(Debug)]
struct Args {
    t: u32,
    n: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    info!("The params are {:?}", args);

    let params = Parameters { t: args.t, n: args.n};

    // Participant = public, Coefficients = private
    let mut participants: Vec<(Participant, Coefficients)> = Vec::new();

    // Participants initialization
    for i in 0..params.n {
        participants.push(Participant::new(&params, i));
    }

    // For each participant, we generate keys to share with the others
    let mut states: Vec<DistributedKeyGeneration<RoundOne>> = Vec::new();
    let mut their_secret_shares: Vec<_> = Vec::new();
    for (i, share) in participants.iter().enumerate()
    {
        // We create a vector of all the other participants
        let mut others: Vec<Participant> = participants.iter()
            .enumerate()
            .filter_map(|(j, participant)| if i != j { Some(participant.0.clone()) } else { None })
            .collect();

        // Round 1 key generation from other participants for a given user
        let result = DistributedKeyGeneration::<RoundOne>::new(&params, &share.0.index, &share.1, &mut others);
        // note: can we use ? instead of match here? couldn't probably because the use of vectors who need clones
        match result {
            Ok(state) =>
            {
                states.push(state.clone());

                let result = state.their_secret_shares();
                match result {
                    Ok(secret_share) => 
                    {
                        // collection of shares for each participant
                        their_secret_shares.push(secret_share.clone());
                    }
                    Err(errors) => {
                        warn!("Failed to create their secret shares: {:?}", errors);
                    }
                }
            },
            Err(errors) => {
                warn!("Failed to create DistributedKeyGeneration: {:?}", errors);
            }
        }
    }

    // For each participant, we generate keys to share with the others
    let mut keys: Vec<(SecretKey, GroupKey)> = Vec::new();
    for (i, state) in states.iter().enumerate()
    {
        // We create a vector of all the other participants' secret shares for a given participant
        let my_secret_shares: Vec<_> =  their_secret_shares.iter().enumerate()
            .filter_map(|(j, s)|
            {
                // order is important here because their_secret_shares is n-1 long and shares are pushed following participants' order
                // TODO: write unit tests for this
                if i < j {
                    Some(s[i].clone())
                } else if i > j { 
                    Some(s[i - 1].clone())
                } else { None }
            })
            .collect();

        // Round 2 key generation from other participants' secret shares for a given participant
        let result = state.clone().to_round_two(my_secret_shares);
        match result {
            Ok(state) =>
            {
                let result = state.finish(participants[i].0.public_key().unwrap());
                match result {
                    Ok((group_key, secret_key)) =>
                    {
                        // collection of keys for each participant
                        keys.push((secret_key, group_key));
                    },
                    Err(errors) => {
                        warn!("Failed to finish: {:?}", errors);
                    }
                }
            },
            Err(errors) => {
                warn! ("Failed to create round two: {:?}", errors);
            }
        }
    }
    
    // assert that all group keys are identical
    let first_group_key = keys[0].1;
    assert!(keys.iter().all(|(_, group_key)| *group_key == first_group_key));

    // To be continued... Thanks for reading!

    Ok(())
}