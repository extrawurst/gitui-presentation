// crossbeam select

fn select_event(
    rx_input: &Receiver<Vec<QueueEvent>>,
    rx_git: &Receiver<AsyncNotification>,
    ...
) -> Result<Vec<QueueEvent>> {
    let mut events: Vec<QueueEvent> = Vec::new();
    
    let mut sel = Select::new();
    sel.recv(rx_input);
    sel.recv(rx_git);
    ...

    let oper = sel.select();

    match oper.index() {
        0 => oper.recv(rx_input).map(|inputs| events.extend(inputs)),
        1 => oper
            .recv(rx_git)
            .map(|ev| events.push(QueueEvent::GitEvent(ev))),
        ...
    }?;

    Ok(events)
}