// The words of the move-again chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-move-again.cfg. It starts as the move chart does, and uses the move chart's words. You move
// the conversation, then move it again while Undo shows or after Undo goes away. While Undo shows, a
// second move is the same move: Undo still goes back to patient A, and a move back to patient A ends
// it. It leaves out new messages, opening and closing the conversation, and the other physician
// moving it.
export { title, state, step, group, colors } from "./chart-move.mjs";
