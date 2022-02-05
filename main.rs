//This program will create a simple neural network
fn main() {
//Creation of 2 neuron input layer. Each neuron will have
//3 variables. One will be input variable, another will be 
//bias and the third will be output.
//Each neuron will be created individually.

//////////////////////  NEURON11 //////////////////////////////
    let neuron11_bias: f32 = 0.0;
	
	let neuron11_input: f32 = 1.0 + neuron11_bias;

	let neuron11_output: f32 = neuron11_input;
	
	//Creation of weights between neuron11 of input layer 
	//to hidden layer. The hidden layer consists of 3 neurons
	//and hence will need 3 weights from neuron11 of input layer
	//to each neuron of the hidden layer
	
	let weight11_21: f32 = 0.001;
	let weight11_22: f32 = 0.002;
	let weight11_23: f32 = 0.004;
	
	
	
//////////////////////////  NEURON12 ///////////////////////////	
	let neuron12_bias: f32 = 0.0;
	
	let neuron12_input: f32 = 1.0 + neuron12_bias;
	
	let neuron12_output: f32 = neuron12_input;
    //Creation of weights between neuron12 of input layer to hidden
	//layer.
	let weight12_21: f32 = 0.016;
	let weight12_22: f32 = 0.012;
	let weight12_23: f32 = 0.007;

/////////////////////HIDDEN LAYER//////////////////////////////////
///////////////////////////NEURON21////////////////////////////////

	let neuron21_bias: f32 = 0.03;
	let neuron21_input: f32 = (neuron11_output * weight11_21) + 
	                          (neuron12_output * weight12_21) + neuron21_bias;
							  
	println!("{}", neuron21_input);					  
	let neuron21_output: f32 = 1.0/(1.0 + (-neuron21_input.exp()));
	println!("{}", neuron21_output);
	let exp_value:f32 = 0.047;
	println!("{}", 1.0/(1.0 + (exp(-exp_value as f64))));

    println!("Hello, world!");
}






