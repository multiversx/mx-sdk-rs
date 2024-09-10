import { configureStore, createSlice } from '@reduxjs/toolkit';

const loadState = () => {
    try {
      const serializedState = localStorage.getItem('scAddress');
      if (serializedState === null) {
        return { contract_address: ''};
      }
      return { contract_address: JSON.parse(serializedState) };  
    } catch (err) {
      return { contract_address: '' };
    }
  };
  
  const saveState = (state) => {
    try {
      const serializedState = JSON.stringify(state.contract_address);
      localStorage.setItem('scAddress', serializedState);
    } catch (err) {
      console.error("Error saving state to localStorage", err);
    }
  };

const scAddressSlice = createSlice({
    name: 'scAddress',
    initialState: loadState(),

    reducers: {
        setScAddress(state, action) {
            state.contract_address = action.payload;
            saveState(state);
        },
        clearScAddress(state) { 
            state.contract_address = '';
        }
    }
});

export const { setScAddress, clearScAddress } = scAddressSlice.actions;

const store = configureStore({
    reducer: {
        scAddress: scAddressSlice.reducer
    },
});

export default store;
