use std::io;

use tokio_i3ipc::{msg, I3};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let mut i3 = I3::connect().await?;
    let worksp = i3.get_workspaces().await?;

    // Get visible workspaces
    let visible_workspaces = worksp.iter().filter(|w| w.visible);

    // Return a tuple of outputs and visible workspaces
    let (mut outputs, workspaces): (Vec<String>, Vec<_>) =
        visible_workspaces.map(|w| (w.output.clone(), w)).unzip();

    let focused_output = worksp
        .iter()
        .filter(|w| w.focused)
        .next()
        .expect("no focused workspace");

    outputs.rotate_right(1);
    let new_layout = outputs
        .into_iter()
        .zip(workspaces)
        .map(|(output, workspace)| {
            let mut ws = workspace.clone();
            ws.output = output;
            ws
        });

    // Move the workspace to the new output
    // for ws in new_layout.clone() {
    //     i3.send_msg_body(
    //         msg::Msg::RunCommand,
    //         format!(
    //             "[workspace={workspace}] focus; move workspace to output {output}",
    //             workspace = ws.name,
    //             output = ws.output
    //         ),
    //     )
    //     .await?;
    // }

    let command = new_layout.into_iter().fold(String::new(), |acc, ws| {
        if acc.is_empty() {
            format!(
                "[workspace={workspace}] focus; move workspace to output {output}",
                workspace = ws.name,
                output = ws.output
            )
        } else {
            format!(
                "{}; [workspace={workspace}] focus; move workspace to output {output}",
                acc,
                workspace = ws.name,
                output = ws.output
            )
        }
    });

    i3.send_msg_body(
        msg::Msg::RunCommand,
        format!(
            "{}; focus output {focused_output}",
            command,
            focused_output = focused_output.output
        ),
    )
    .await?;

    Ok(())
}
