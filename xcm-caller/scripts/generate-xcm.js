import { westendAssetHub, XcmV3MultiassetFungibility, XcmV5AssetFilter, XcmV5Instruction, XcmV5Junction, XcmV5Junctions, XcmV5WildAsset, XcmVersionedXcm } from "@polkadot-api/descriptors";
import { Binary, Enum, getTypedCodecs } from "polkadot-api";

const PAS_UNITS = 10_000_000_000n;
const PAS_CENTS = 100_000_000n;
async function createTeleportMessage(paraId, beneficiary, amount) {
    const xcm = XcmVersionedXcm.V5([
        XcmV5Instruction.WithdrawAsset([
            {
                id: { parents: 1, interior: XcmV5Junctions.Here() },
                fun: XcmV3MultiassetFungibility.Fungible(amount * PAS_UNITS),
            }
        ]),
        XcmV5Instruction.PayFees({
            asset: {
                id: { parents: 1, interior: XcmV5Junctions.Here() },
                fun: XcmV3MultiassetFungibility.Fungible(100n * PAS_UNITS),
            }
        }),
        XcmV5Instruction.InitiateTransfer({
            destination: { parents: 1, interior: XcmV5Junctions.X1(XcmV5Junction.Parachain(paraId)) },
            remote_fees: Enum(
                "Teleport",
                XcmV5AssetFilter.Definite([
                    {
                        id: { parents: 1, interior: XcmV5Junctions.Here() },
                        fun: XcmV3MultiassetFungibility.Fungible(10n * PAS_CENTS),
                    }
                ]),
                
            ),
            preserve_origin: false,
            remote_xcm: [
                XcmV5Instruction.DepositAsset({
                    assets: XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1)),
                    beneficiary: {
                        parents: 0,
                        interior: XcmV5Junctions.X1(XcmV5Junction.AccountId32({
                            id: beneficiary,
                            network: undefined,
                        }))
                    }
                })
            ],
            assets: [
                Enum("Teleport", XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1))),
            ]
        })
    ]);

    const codecs = await getTypedCodecs(westendAssetHub);
    const xcmEncoded = codecs.apis.XcmPaymentApi.query_xcm_weight.args.enc([xcm]);
    const xcmHex = Binary.fromBytes(xcmEncoded).asHex();

    console.log({ xcmHex });
    return xcmHex;
}

createTeleportMessage(
    1300, 
    Binary.fromHex("0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"),
    355n
)
  .then((result) => {
    console.log(result);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });