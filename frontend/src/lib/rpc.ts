import { createChannel, createClient } from 'nice-grpc-web';
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

	const opts: any = {};
	const authToken = get(tstatsAuthToken);
	if (authToken === null) {
		opts['Authorization'] = 'Bearer ' + authToken;
	}

	return createClient(definition, channel, {
		'*': opts
	});
}
