<script lang="ts">
    import Button from "../components/button.svelte";

    const socket = new WebSocket("ws://127.0.0.1:8080", "kek");
    socket.binaryType = "arraybuffer";

    async function handleClick() {
        const stream = await window
            .navigator
            .mediaDevices
            .getUserMedia({ audio: true });

        const mediaRecorder = new MediaRecorder(stream);

        mediaRecorder.addEventListener("dataavailable", (chunk) => {
            chunk.data.arrayBuffer()
                .then((data) => {
                    socket.send(data);
                }).catch(console.error);
        });

        mediaRecorder.start(500);
    }

    const audioCtx = new AudioContext();
    const destinationNode = audioCtx.createMediaStreamDestination();

    socket.addEventListener("message", (evt) => {
        const audioData = evt.data;
            
        destinationNode.context.decodeAudioData(audioData, (buffer) => {
            let source = destinationNode.context.createBufferSource();
            source.buffer = buffer;
            source.connect(destinationNode);
            source.start(0);
        }, (err) => {
            console.error(err)
        });
    });

</script>

<section>
    <Button on:click={handleClick}>Start</Button>
</section>

