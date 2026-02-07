use std::fmt;

/// Represents a transaction in the blockchain
/// Transfers amount from sender to receiver
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

impl Transaction {
    /// Creates a new transaction with validation
    pub fn new(sender: String, receiver: String, amount: f64) -> Result<Self, String> {
        // Validate transaction
        if sender.is_empty() {
            return Err("Sender cannot be empty".to_string());
        }
        if receiver.is_empty() {
            return Err("Receiver cannot be empty".to_string());
        }
        if sender == receiver {
            return Err("Sender and receiver cannot be the same".to_string());
        }
        if amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        }

        Ok(Transaction {
            sender,
            receiver,
            amount,
        })
    }

    /// Creates a transaction without validation (for testing only)
    #[cfg(test)]
    pub fn new_unvalidated(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} : {:.2}",
            self.sender, self.receiver, self.amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transaction() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.sender, "Alice");
        assert_eq!(tx.receiver, "Bob");
        assert_eq!(tx.amount, 10.0);
    }

    #[test]
    fn test_empty_sender_rejected() {
        let tx = Transaction::new(
            String::from(""),
            String::from("Bob"),
            10.0,
        );
        assert!(tx.is_err());
    }

    #[test]
    fn test_empty_receiver_rejected() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from(""),
            10.0,
        );
        assert!(tx.is_err());
    }

    #[test]
    fn test_self_transaction_rejected() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from("Alice"),
            10.0,
        );
        assert!(tx.is_err());
    }

    #[test]
    fn test_zero_amount_rejected() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from("Bob"),
            0.0,
        );
        assert!(tx.is_err());
    }

    #[test]
    fn test_negative_amount_rejected() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from("Bob"),
            -10.0,
        );
        assert!(tx.is_err());
    }

    #[test]
    fn test_transaction_display() {
        let tx = Transaction::new(
            String::from("Alice"),
            String::from("Bob"),
            10.5,
        ).unwrap();
        let display = format!("{}", tx);
        assert!(display.contains("Alice"));
        assert!(display.contains("Bob"));
        assert!(display.contains("10.50"));
    }

    #[test]
    fn test_transaction_clone() {
        let tx1 = Transaction::new(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        ).unwrap();
        let tx2 = tx1.clone();
        assert_eq!(tx1, tx2);
    }
}
