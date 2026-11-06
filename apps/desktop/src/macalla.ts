// Macalla fork switches. Upstream behaviour is the `false` side of each flag.
export const macallaFork = {
  // Transcription and summaries run only when asked for: Re-transcribe on the
  // Transcript tab, Generate on the Summary tab. Nothing runs by itself after
  // a recording stops, after an upload, or when the app relaunches. Tests set
  // this to false (see test-setup.ts) so upstream's suites keep their meaning.
  onDemandProcessing: true,
};
