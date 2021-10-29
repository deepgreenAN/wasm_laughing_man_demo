<script>
	export let open = false
</script>

<button class="hambergur-button" class:open on:click={() => open = !open}>
	<svg width=32 height=24>
		<line id="top" x1=0 y1=2  x2=32 y2=2/>
		<line id="middle" x1=0 y1=12 x2=32 y2=12/>
		<line id="bottom" x1=0 y1=22 x2=32 y2=22/>
	</svg>
</button>

<style>
    .hambergur-button {
        color: #bfbfbf;
    }

    .hambergur-button :hover {
        color: #8a8a8a;
    }

    .hambergur-button :focus {
    	outline: 2px solid transparent;
        outline-offset: 2px;
    }

	svg {
		min-height: 24px;
		transition: transform 0.3s ease-in-out;
	}

	@media (min-width: 320px) and (max-width: 768px) {
		svg {
			visibility: hidden;
		}
	}
	
	svg line {
		stroke: currentColor;
		stroke-width: 3;
		transition: transform 0.3s ease-in-out
	}
	
	button {
		background-color: rgba(0,0,0,0);
		z-index: 40;
        cursor: pointer;
		border-style: none;
	}

	@media (min-width: 320px) and (max-width: 768px) {
		button {
			visibility: hidden;
		}
	}

	button :active {
		background-color: rgba(214,214,214,0.5);
	}
	
	.open svg {
		transform: scale(0.8)
	}
	
	.open #top {
		transform: translate(6px, 0px) rotate(45deg)
	}
	
	.open #middle {
		opacity: 0;
	}
	
  	.open #bottom {
		transform: translate(-12px, 9px) rotate(-45deg)
	}
</style>