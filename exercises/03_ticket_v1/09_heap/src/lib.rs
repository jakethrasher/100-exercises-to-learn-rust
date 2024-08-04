pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // this is a tricky question!
        // the "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // if you're curious, check out the "data layout" section of the rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
