import createActor, { getProfileCanisterId } from './utils/canisters/profile';
import { principal } from './utils/agent';

describe('Profile', () => {
  let profileActor: ReturnType<typeof createActor>;

  beforeAll(async () => {
    const profileId = await getProfileCanisterId();
    profileActor = createActor(profileId);
  });

  describe('Health-check', () => {
    it('should have correct name', async () => {
      const name = await profileActor.name();
      expect(name).toBe('Profile Canister');
    });
  });

  describe('Services', () => {
    describe('Set Display Name', () => {
      it('should update the display name correctly', async () => {
        const setDisplayName = await profileActor.set_display_name({ name: 'Barry Allen' });
        expect(setDisplayName).toEqual(null);

        const getProfile = await profileActor.get_profile();
        expect(getProfile.display_name).toEqual('Barry Allen');
      });
    });
  });
});
