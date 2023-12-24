# Nyx Trajectory Visualization Examples
This example is a self-contained crates that implement some of the examples
shown in the docs: [Lunar transfer](https://nyxspace.com/nyxspace/showcase/mission_design/lunar_transfer/),
[Orbit design from a genetic algorithm](https://nyxspace.com/nyxspace/showcase/mission_design/orbit_design_ga/).
It also demonstrates displaying the results with [egui](https://www.egui.rs/).

I don't think Nyx needs a front end, but here's a way to do it if you want one.

## Motivation
I was very annoyed by the workflow needed to visualize trajectories and orbits.
In order to generate plots, I had set up a sim, generate a trajectory, export
the trajectory to Parquet files, read the Parquet files to a dataframe, and then
plot the dataframe in Plotly. Yikes.

I was impressed by the plot demos on the egui demo page and was curious to see
if orbits and trajectories could be plotted straight away as an egui plot.

## Approach
- **Native GUI with
  [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).** WASM is
  intimidating, but egui runs natively too. I did some experiments with Bevy but
  it was a _loooot_ of overhead. Since Nyx is basically its own physics engine,
  a simple UI with line plots should be enough for now. `eframe` is a simple GUI
  framework that is the backend for the [egui demo](https://www.egui.rs/) that I
  liked, so I went with that.
- **Reproduce Python visualizations and documented demos.** Self explanatory.
  It's nice to start from a baseline instead of potentially introducing new bugs
  from making my own scenario from scratch.
- **As a self-contained crate.** Nyx has no examples, and the GUI doesn't change
  any core functions of Nyx. It makes more sense to provide this as an
  interactive example for folks curious about Nyx to build and learn from
  themselves.

## Results
The proof of concept is there, but it's rough.
- [x] Able to change simulation inputs from the GUI
- [ ] Orbital parameters change as orbit is propagated.
- [ ] Trajectories plotted as a set of line plots.
- [ ] Trajectories plotted in 3D.

## Future work
- Real-time plotting as propagations progress
- Compiling to WASM and embedding in the nyx-space docs as an interactive demo.

# Demonstration
