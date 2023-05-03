<script lang="ts">
    import Button from "../components/button.svelte";

    function handleClick() {
        const audioCtx = new AudioContext();
        const audio = new Audio("/someday-that-summer.wav");
        const source = audioCtx.createMediaElementSource(audio);

        const node = audioCtx.createAnalyser()
        node.fftSize = 2 ** 7;
        source.connect(node);

        const dataArray = new Float32Array(node.fftSize);

        function play() {
            const bufferSource = audioCtx.createBuffer(2, audioCtx.sampleRate * 30, audioCtx.sampleRate);
            node.getFloatTimeDomainData(dataArray);
            bufferSource.copyToChannel(dataArray, 0, 0);
            bufferSource.copyToChannel(dataArray, 1, 0);
            const soundSource = audioCtx.createBufferSource();
            soundSource.buffer = bufferSource;
            soundSource.connect(audioCtx.destination);
            soundSource.start(0);
            console.log("Data", dataArray);
            setTimeout(play, 5);
        }

        play();



        audio.play();
    }
</script>

<section>
    <Button on:click={handleClick}>Start</Button>
</section>

