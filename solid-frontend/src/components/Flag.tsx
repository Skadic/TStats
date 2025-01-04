import { Country } from "../lib/api/v1types";
import { Component } from "../lib/types";
import "/node_modules/flag-icons/css/flag-icons.min.css";

const Flag: Component<{ country: Country }> = (props) => {
	const style = `bg-center flag fi fi-${props.country.countryCode.toLowerCase()}`;
	return <span class={style}></span>;
};

export default Flag;
