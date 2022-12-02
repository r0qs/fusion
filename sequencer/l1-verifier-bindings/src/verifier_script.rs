pub use verifier_script::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod verifier_script {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "VerifierScript was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[{\"internalType\":\"contract Verifier\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static VERIFIERSCRIPT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static VERIFIERSCRIPT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526200000e62000032565b6007556009805460ff191660011790553480156200002b57600080fd5b5062001249565b6040805160a08101825260056060820181815264105b9d9a5b60da1b60808401528252617a6960208084019190915283518085018552601581527f687474703a2f2f3132372e302e302e313a3835343500000000000000000000009181019190915282840152915164185b9d9a5b60da1b81526000926008910190815260405190819003602001902081518190620000cb908262000ef7565b506020820151600182015560408201516002820190620000ec908262000ef7565b50506040805160a0810182526007606082018181526612185c991a185d60ca1b60808401528252617a6960208084019190915283518085018552601581527f687474703a2f2f3132372e302e302e313a38353435000000000000000000000091810191909152828401529151661a185c991a185d60ca1b815290925060089101908152604051908190036020019020815181906200018b908262000ef7565b506020820151600182015560408201516002820190620001ac908262000ef7565b5090505060405180606001604052806040518060400160405280600781526020016613585a5b9b995d60ca1b8152508152602001600181526020016040518060600160405280603d81526020016200208c603d91399052604051661b585a5b9b995d60ca1b81526008906007019081526040519081900360200190208151819062000238908262000ef7565b50602082015160018201556040820151600282019062000259908262000ef7565b50905050604051806060016040528060405180604001604052806006815260200165476f65726c6960d01b8152508152602001600581526020016040518060600160405280603c8152602001620020c9603c9139905260405165676f65726c6960d01b815260089060060190815260405190819003602001902081518190620002e3908262000ef7565b50602082015160018201556040820151600282019062000304908262000ef7565b50506040805160a081018252600760608201818152665365706f6c696160c81b6080840152825262aa36a760208084019190915283518085018552601781527f68747470733a2f2f7270632e7365706f6c69612e64657600000000000000000091810191909152828401529151667365706f6c696160c81b81529092506008910190815260405190819003602001902081518190620003a4908262000ef7565b506020820151600182015560408201516002820190620003c5908262000ef7565b50506040805160a081018252600860608201818152674f7074696d69736d60c01b60808401528252600a60208084019190915283518085018552601b81527f68747470733a2f2f6d61696e6e65742e6f7074696d69736d2e696f000000000091810191909152828401529151676f7074696d69736d60c01b815290925081019081526040519081900360200190208151819062000463908262000ef7565b50602082015160018201556040820151600282019062000484908262000ef7565b50506040805160a081018252600f606082018181526e4f7074696d69736d20476f65726c6960881b608084015282526101a460208084019190915283518085018552601a81527f68747470733a2f2f676f65726c692e6f7074696d69736d2e696f000000000000918101919091528284015291516e6f7074696d69736d5f676f65726c6960881b8152909250600891019081526040519081900360200190208151819062000533908262000ef7565b50602082015160018201556040820151600282019062000554908262000ef7565b50506040805160a081018252600c606082018181526b417262697472756d204f6e6560a01b6080840152825261a4b160208084019190915283518085018552601c81527f68747470733a2f2f617262312e617262697472756d2e696f2f72706300000000918101919091528284015291516b617262697472756d5f6f6e6560a01b81529092506008910190815260405190819003602001902081518190620005fd908262000ef7565b5060208201516001820155604082015160028201906200061e908262000ef7565b5090505060405180606001604052806040518060400160405280601381526020017f417262697472756d204f6e6520476f65726c6900000000000000000000000000815250815260200162066eed8152602001604051806060016040528060258152602001620021056025913990526040517f617262697472756d5f6f6e655f676f65726c6900000000000000000000000000815260089060130190815260405190819003602001902081518190620006d8908262000ef7565b506020820151600182015560408201516002820190620006f9908262000ef7565b50506040805160a081018252600d606082018181526c417262697472756d204e6f766160981b6080840152825261a4ba60208084019190915283518085018552601c81527f68747470733a2f2f6e6f76612e617262697472756d2e696f2f72706300000000918101919091528284015291516c617262697472756d5f6e6f766160981b81529092506008910190815260405190819003602001902081518190620007a4908262000ef7565b506020820151600182015560408201516002820190620007c5908262000ef7565b50506040805160a081018252600760608201818152662837b63cb3b7b760c91b60808401528252608960208084019190915283518085018552601781527f68747470733a2f2f706f6c79676f6e2d7270632e636f6d00000000000000000091810191909152828401529151663837b63cb3b7b760c91b8152909250600891019081526040519081900360200190208151819062000863908262000ef7565b50602082015160018201556040820151600282019062000884908262000ef7565b50506040805160a081018252600e606082018181526d506f6c79676f6e204d756d62616960901b608084015282526201388160208084019190915283518085018552601e81527f68747470733a2f2f7270632d6d756d6261692e6d617469632e746f6461790000918101919091528284015291516d706f6c79676f6e5f6d756d62616960901b8152909250600891019081526040519081900360200190208151819062000932908262000ef7565b50602082015160018201556040820151600282019062000953908262000ef7565b509050506040518060600160405280604051806040016040528060098152602001684176616c616e63686560b81b815250815260200161a86a815260200160405180606001604052806025815260200162002067602591399052604051686176616c616e63686560b81b815260089060090190815260405190819003602001902081518190620009e4908262000ef7565b50602082015160018201556040820151600282019062000a05908262000ef7565b5090505060405180606001604052806040518060400160405280600e81526020016d4176616c616e6368652046756a6960901b815250815260200161a86981526020016040518060600160405280602a81526020016200212a602a913990526040516d6176616c616e6368655f66756a6960901b8152600890600e019081526040519081900360200190208151819062000aa0908262000ef7565b50602082015160018201556040820151600282019062000ac1908262000ef7565b5090505060405180606001604052806040518060400160405280600f81526020016e2127211029b6b0b93a1021b430b4b760891b815250815260200160388152602001604051806060016040528060218152602001620021546021913990526040516e3137312fb9b6b0b93a2fb1b430b4b760891b8152600890600f019081526040519081900360200190208151819062000b5d908262000ef7565b50602082015160018201556040820151600282019062000b7e908262000ef7565b5090505060405180606001604052806040518060400160405280601781526020017f424e4220536d61727420436861696e20546573746e65740000000000000000008152508152602001606181526020016040518060600160405280602e815260200162002039602e913990526040517f626e625f736d6172745f636861696e5f746573746e657400000000000000000081526008906017019081526040519081900360200190208151819062000c36908262000ef7565b50602082015160018201556040820151600282019062000c57908262000ef7565b50506040805160a081018252600c606082018181526b23b737b9b4b99021b430b4b760a11b60808401528252606460208084019190915283518085018552601b81527f68747470733a2f2f7270632e676e6f736973636861696e2e636f6d0000000000918101919091528284015291516b33b737b9b4b9afb1b430b4b760a11b8152909250600891019081526040519081900360200190208151819062000cff908262000ef7565b50602082015160018201556040820151600282019062000d20908262000ef7565b5090505060007f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b0316639d2ad72a6040518163ffffffff1660e01b8152600401600060405180830381865afa15801562000d88573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405262000db29190810190620010bb565b905060005b815181101562000e495781818151811062000dd65762000dd6620011ed565b602002602001015160200151600883838151811062000df95762000df9620011ed565b60200260200101516000015160405162000e14919062001203565b9081526020016040518091039020600201908162000e33919062000ef7565b508062000e408162001221565b91505062000db7565b50600091505090565b634e487b7160e01b600052604160045260246000fd5b600181811c9082168062000e7d57607f821691505b60208210810362000e9e57634e487b7160e01b600052602260045260246000fd5b50919050565b601f82111562000ef257600081815260208120601f850160051c8101602086101562000ecd5750805b601f850160051c820191505b8181101562000eee5782815560010162000ed9565b5050505b505050565b81516001600160401b0381111562000f135762000f1362000e52565b62000f2b8162000f24845462000e68565b8462000ea4565b602080601f83116001811462000f63576000841562000f4a5750858301515b600019600386901b1c1916600185901b17855562000eee565b600085815260208120601f198616915b8281101562000f945788860151825594840194600190910190840162000f73565b508582101562000fb35787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b604080519081016001600160401b038111828210171562000fe85762000fe862000e52565b60405290565b604051601f8201601f191681016001600160401b038111828210171562001019576200101962000e52565b604052919050565b60005b838110156200103e57818101518382015260200162001024565b50506000910152565b600082601f8301126200105957600080fd5b81516001600160401b0381111562001075576200107562000e52565b6200108a601f8201601f191660200162000fee565b818152846020838601011115620010a057600080fd5b620010b382602083016020870162001021565b949350505050565b60006020808385031215620010cf57600080fd5b82516001600160401b0380821115620010e757600080fd5b818501915085601f830112620010fc57600080fd5b81518181111562001111576200111162000e52565b8060051b6200112285820162000fee565b91825283810185019185810190898411156200113d57600080fd5b86860192505b83831015620011e0578251858111156200115d5760008081fd5b86016040818c03601f1901811315620011765760008081fd5b6200118062000fc3565b8983015188811115620011935760008081fd5b620011a38e8c8387010162001047565b825250908201519087821115620011ba5760008081fd5b620011ca8d8b8486010162001047565b818b015284525050918601919086019062001143565b9998505050505050505050565b634e487b7160e01b600052603260045260246000fd5b600082516200121781846020870162001021565b9190910192915050565b6000600182016200124257634e487b7160e01b600052601160045260246000fd5b5060010190565b610de080620012596000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80630a9254e414610046578063c040622614610048578063f8ccbf471461006d575b600080fd5b005b61005061008a565b6040516001600160a01b0390911681526020015b60405180910390f35b60095461007a9060ff1681565b6040519015158152602001610064565b60007f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b0316637fb5297f6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156100ea57600080fd5b505af11580156100fe573d6000803e3d6000fd5b505050506000604051610110906101ab565b604051809103906000f08015801561012c573d6000803e3d6000fd5b5090507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b03166376eadd366040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561018d57600080fd5b505af11580156101a1573d6000803e3d6000fd5b5092949350505050565b610bf2806101b98339019056fe608060405234801561001057600080fd5b5060006020819052620f4240600080516020610bd283398151915281905573419978a8729ed2c3b1048b5bba49f8599ed8f7c1909152600080516020610bb28339815191525561005e610063565b6100d6565b60006020818152600080516020610bd28339815191525473419978a8729ed2c3b1048b5bba49f8599ed8f7c1909252600080516020610bb2833981519152546040516100b9939201918252602082015260400190565b60408051601f198184030181529190528051602090910120600155565b610acd806100e56000396000f3fe60806040526004361061004a5760003560e01c80630c3f6acf1461004f5780632e1a7d4d1461007a578063d0e30db01461009c578063ebf0c717146100a4578063f74aa465146100c8575b600080fd5b34801561005b57600080fd5b506100646100e8565b60405161007191906107bb565b60405180910390f35b34801561008657600080fd5b5061009a6100953660046107ec565b610167565b005b61009a610210565b3480156100b057600080fd5b506100ba60015481565b604051908152602001610071565b3480156100d457600080fd5b5061009a6100e3366004610891565b6102c8565b6100f0610757565b50604080518082019091527f7028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fe54815273419978a8729ed2c3b1048b5bba49f8599ed8f7c1600090815260209081527fd7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b4549082015290565b6040805160608101825260018082523360208301529181018390526002805480840182556000829052825191027f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace01805492939092839160ff199091169083818111156101d6576101d6610a16565b0217905550602082015181546001600160a01b0390911661010002610100600160a81b031990911617815560409091015160019091015550565b6000341161021d57600080fd5b6040805160608101825260008082523360208301523492820192909252600280546001808201835593829052825191027f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace01805492939092839160ff1990911690838181111561028f5761028f610a16565b0217905550602082015181546001600160a01b0390911661010002610100600160a81b0319909116178155604090910151600190910155565b60005b8251811015610345576102f68382815181106102e9576102e9610a2c565b602002602001015161037c565b61031357604051638baa579f60e01b815260040160405180910390fd5b61033583828151811061032857610328610a2c565b60200260200101516103bf565b61033e81610a58565b90506102cb565b5061034e61042d565b600154811461037057604051630b6fac0360e41b815260040160405180910390fd5b6103786104c4565b5050565b60008061039061038b84610521565b610596565b905082600001516001600160a01b03166103ae8285608001516105d1565b6001600160a01b0316149392505050565b60408082015182516001600160a01b0316600090815260208190529182208054919290916103ee908490610a71565b90915550506040808201516020808401516001600160a01b0316600090815290819052918220805491929091610425908490610a84565b909155505050565b600060208181527f7028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fe5473419978a8729ed2c3b1048b5bba49f8599ed8f7c19092527fd7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b4546040516104a7939201918252602082015260400190565b60408051601f198184030181529190528051602090910120600155565b60005b60025481101561050a576104fa600282815481106104e7576104e7610a2c565b9060005260206000209060020201610651565b61050381610a58565b90506104c7565b5061051760026000610775565b61051f61042d565b565b600081600001518260200151836040015184606001516040516020016105799493929190606094851b6bffffffffffffffffffffffff1990811682529390941b90921660148401526028830152604882015260680190565b604051602081830303815290604052805190602001209050919050565b6040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c8101829052600090605c01610579565b6000806000806105e085610728565b6040805160008152602081018083528b905260ff8516918101919091526060810183905260808101829052929550909350915060019060a0016020604051602081039080840390855afa15801561063b573d6000803e3d6000fd5b5050506020604051035193505050505b92915050565b6000815460ff16600181111561066957610669610a16565b036106ac576001810154815461010090046001600160a01b0316600090815260208190526040812080549091906106a1908490610a84565b909155506107259050565b6001810154815461010090046001600160a01b0316600090815260208190526040812080549091906106df908490610a71565b9091555050805460018201546040516101009092046001600160a01b0316916108fc82150291906000818181858888f19350505050158015610378573d6000803e3d6000fd5b50565b6000806000835160411461073b57600080fd5b5050506020810151604082015160609092015160001a92909190565b60405180604001604052806002906020820280368337509192915050565b508054600082556002029060005260206000209081019061072591905b808211156107b75780546001600160a81b031916815560006001820155600201610792565b5090565b60408101818360005b60028110156107e35781518352602092830192909101906001016107c4565b50505092915050565b6000602082840312156107fe57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b60405160a0810167ffffffffffffffff8111828210171561083e5761083e610805565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561086d5761086d610805565b604052919050565b80356001600160a01b038116811461088c57600080fd5b919050565b600080604083850312156108a457600080fd5b823567ffffffffffffffff808211156108bc57600080fd5b818501915085601f8301126108d057600080fd5b81356020828211156108e4576108e4610805565b8160051b6108f3828201610844565b928352848101820192828101908a85111561090d57600080fd5b83870192505b84831015610a055782358681111561092a57600080fd5b8701601f1960a0828e038201121561094157600080fd5b61094961081b565b610954878401610875565b815261096260408401610875565b87820152606083013560408201526080830135606082015260a08301358981111561098c57600080fd5b8084019350508d603f8401126109a157600080fd5b86830135898111156109b5576109b5610805565b6109c58884601f84011601610844565b92508083528e60408286010111156109dc57600080fd5b806040850189850137600090830188015260808101919091528352509183019190830190610913565b9a9890920135985050505050505050565b634e487b7160e01b600052602160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060018201610a6a57610a6a610a42565b5060010190565b8181038181111561064b5761064b610a42565b8082018082111561064b5761064b610a4256fea264697066735822122035a43ab4190753fec9826941668114dd7bf056b1ea8ca9247b285c44a7cbd03764736f6c63430008110033d7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b47028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fea26469706673582212208ffd67a80f6088ff89d94cdf889625c75aca410f25b3ad90fb16c77646e8299864736f6c6343000811003368747470733a2f2f646174612d736565642d7072656273632d312d73312e62696e616e63652e6f72673a3835343568747470733a2f2f6170692e617661782e6e6574776f726b2f6578742f62632f432f72706368747470733a2f2f6d61696e6e65742e696e667572612e696f2f76332f363737303435346263366561343263353861616331323937383533316239336668747470733a2f2f676f65726c692e696e667572612e696f2f76332f363737303435346263366561343263353861616331323937383533316239336668747470733a2f2f676f65726c692d726f6c6c75702e617262697472756d2e696f2f72706368747470733a2f2f6170692e617661782d746573742e6e6574776f726b2f6578742f62632f432f72706368747470733a2f2f6273632d6461746173656564312e62696e616e63652e6f7267" . parse () . expect ("invalid bytecode")
        });
    pub struct VerifierScript<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VerifierScript<M> {
        fn clone(&self) -> Self {
            VerifierScript(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VerifierScript<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for VerifierScript<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VerifierScript))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VerifierScript<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VERIFIERSCRIPT_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                VERIFIERSCRIPT_ABI.clone(),
                VERIFIERSCRIPT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `run` (0xc0406226) function"]
        pub fn run(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for VerifierScript<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `run` function with signature `run()` and selector `[192, 64, 98, 38]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VerifierScriptCalls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ethers::core::abi::AbiDecode for VerifierScriptCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierScriptCalls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VerifierScriptCalls::Run(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierScriptCalls::SetUp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VerifierScriptCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VerifierScriptCalls::IsScript(element) => element.encode(),
                VerifierScriptCalls::Run(element) => element.encode(),
                VerifierScriptCalls::SetUp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VerifierScriptCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VerifierScriptCalls::IsScript(element) => element.fmt(f),
                VerifierScriptCalls::Run(element) => element.fmt(f),
                VerifierScriptCalls::SetUp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for VerifierScriptCalls {
        fn from(var: IsScriptCall) -> Self {
            VerifierScriptCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for VerifierScriptCalls {
        fn from(var: RunCall) -> Self {
            VerifierScriptCalls::Run(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for VerifierScriptCalls {
        fn from(var: SetUpCall) -> Self {
            VerifierScriptCalls::SetUp(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsScriptReturn(pub bool);
    #[doc = "Container type for all return fields from the `run` function with signature `run()` and selector `[192, 64, 98, 38]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RunReturn(pub ethers::core::types::Address);
}
