document.addEventListener('DOMContentLoaded', () => {
  const taskType = document.getElementById('task-type');
  const textGroup = document.getElementById('text-group');
  const audioGroup = document.getElementById('audio-group');
  const modelGroup = document.getElementById('model-group');
  const inputText = document.getElementById('input-text');
  const audioFile = document.getElementById('audio-file');
  const whisperModel = document.getElementById('whisper-model');
  const outputPath = document.getElementById('output-path');
  const commandEl = document.getElementById('generated-command');
  const copyBtn = document.getElementById('copy-btn');

  function updateCommand() {
    const task = taskType.value;
    const txt = (inputText.value || '').trim();
    const aud = (audioFile.value || 'speech.wav').trim();
    const mdl = whisperModel.value;
    const out = (outputPath.value || 'output.wav').trim();

    if (task === 'tts') {
      textGroup.style.display = 'block';
      audioGroup.style.display = 'none';
      modelGroup.style.display = 'none';
      commandEl.textContent = `mlx-audio tts -t "${txt}" -o ${out}`;
    } else if (task === 'stt') {
      textGroup.style.display = 'none';
      audioGroup.style.display = 'block';
      modelGroup.style.display = 'block';
      commandEl.textContent = `mlx-audio stt -a ${aud} -m ${mdl}`;
    } else if (task === 'vad') {
      textGroup.style.display = 'none';
      audioGroup.style.display = 'block';
      modelGroup.style.display = 'none';
      commandEl.textContent = `mlx-audio vad -a ${aud} --threshold 0.5`;
    } else if (task === 'codec') {
      textGroup.style.display = 'none';
      audioGroup.style.display = 'block';
      modelGroup.style.display = 'none';
      commandEl.textContent = `mlx-audio codec -i ${aud} -o ${out}`;
    }
  }

  [taskType, inputText, audioFile, whisperModel, outputPath].forEach(el => {
    el.addEventListener('input', updateCommand);
    el.addEventListener('change', updateCommand);
  });

  copyBtn.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(commandEl.textContent);
      copyBtn.textContent = 'Copied!';
      setTimeout(() => {
        copyBtn.textContent = 'Copy';
      }, 2000);
    } catch (e) {
      console.error(e);
    }
  });

  updateCommand();
});
