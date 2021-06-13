use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self { Self { permit } }
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("Ticket freed")
    }
}

impl Museum {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(total)
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let musem = Museum::new(50);

        let ticket = musem.get_ticket().unwrap();
        
        println!("=============");
        println!("avaliable: {}", musem.tickets());

        assert_eq!(musem.tickets(), 49);

        let ticket2 =  musem.get_ticket().unwrap();

        assert_eq!(musem.tickets(), 48);

        let _tickets: Vec<Ticket> = (0..48).map(|_i| musem.get_ticket().unwrap()).collect();
        assert_eq!(musem.tickets(), 0);

        assert!(musem.get_ticket().is_none());

        drop(ticket);
        {
            let ticket = musem.get_ticket().unwrap();
            println!("got ticket: {:?}", ticket);
        }
        println!("!!!!");
    }
}
