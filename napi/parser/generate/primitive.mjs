import {Kind, Niche, registerKindClass} from './common.mjs';

export class Primitive extends Kind {
    initFromDef(def) {
        super.initFromDef(def);
        
        if (this.name === 'Bool') {
            this.niche = new Niche({offset: 0, size: 1, min: 2, max: 255});
        } else if (this.name.startsWith('NonZero')) {
            this.niche = new Niche({offset: 0, size: this.size, min: 0, max: 0});
        }
    }

    generateDeserializerCall(posStr, _deser) {
        return deserializerCallGenerators[this.name](posStr);
    }
}
registerKindClass('primitive', Primitive);

const deserializerCallGenerators = {
    U8: posStr => `uint8[${posStr}]`,
    U32: posStr => `uint32[(${posStr}) >> 2]`,
    F64: posStr => `float64[(${posStr}) >> 3]`,
    Bool: posStr => `uint8[${posStr}] === 1`,
};
