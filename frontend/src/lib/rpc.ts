import { Metadata, createChannel, createClientFactory } from 'nice-grpc-web';
import type { Channel, Client, CompatServiceDefinition } from 'nice-grpc-web';
import { get } from 'svelte/store';
import persisted from './store';
import { BACKEND_URI } from '$lib';

export const tstatsAuthToken = persisted<string | null>('tstatsAuthToken', null);

export function tstatsChannel(): Channel {
	return createChannel(BACKEND_URI);
}

export function tstatsClient<Service extends CompatServiceDefinition>(
	definition: Service,
	channel?: Channel
): Client<Service> {
	if (channel === undefined) {
		channel = tstatsChannel();
	}

	const factory = createClientFactory().use((call, options) => {
		let meta = Metadata(options.metadata);
		const tok = get(tstatsAuthToken);
		if (tok !== null) {
			meta = Metadata(options.metadata).set('authorization', 'Bearer ' + tok);
		}

		return call.next(call.request, {
			...options,
			metadata: meta
		});
	});

	return factory.create(definition, channel);
}
