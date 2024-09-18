import {
	Metadata,
	createChannel,
	createClientFactory,
	ClientError,
	Status,
	type ClientMiddlewareCall
} from 'nice-grpc-web';
import type { Channel, Client, CompatServiceDefinition } from 'nice-grpc-web';
import { get } from 'svelte/store';
import persisted from './store';
import { BACKEND_URI } from '$lib';
import type { CallOptions } from 'nice-grpc-common';

export const tstatsAuthToken = persisted<string | null>('tstatsAuthToken', null);

export function tstatsChannel(): Channel {
	console.log("URI: " + BACKEND_URI)
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
	}).use(loggingMiddleware);

	return factory.create(definition, channel);
}

async function* loggingMiddleware<Request, Response>(
	call: ClientMiddlewareCall<Request, Response>,
	options: CallOptions,
) {
	const {path} = call.method;

	console.log('Client call', path, 'start');

	try {
		const result = yield* call.next(call.request, options);

		console.log('Client call', path, 'end: OK');

		return result;
	} catch (error) {
		if (error instanceof ClientError) {
			console.log(
				'Client call',
				path,
				`end: ${Status[error.code]}: ${error.details}`,
			);
		} else {
			console.log('Client call', path, `error: ${error}`);
		}

		throw error;
	}
}