<script lang="ts">
	import { checkShortcut } from '$lib/general'

	let visible = false
	function open() {
		visible = true
	}
	function close() {
		visible = false
	}
	function toggle() {
		visible = !visible
	}

	function keydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'Escape') && visible) {
			e.preventDefault()
			e.stopPropagation()
			close()
		}
	}
</script>

<div
	class="relative outline-none"
	on:keydown={keydown}
	on:focusout={close}
	tabindex="-1"
	role="button"
>
	<slot {visible} {toggle} {close} {open} />
	<div class="absolute z-10" class:hidden={!visible}>
		<slot name="popup" />
	</div>
</div>
