import createActor, { getProfileCanisterId } from './utils/canisters/profile';
import { principal, fakePrincipal } from './utils/agent';

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

      describe('Set Description', () => {
        it('should update the description correctly', async () => {
          const setDescription = await profileActor.set_description({ description: 'The fastest man alive' });
          expect(setDescription).toEqual(null);

          const getProfile = await profileActor.get_profile();
          expect(getProfile.description).toEqual('The fastest man alive');
        });
      });

      describe('Set Emoji', () => {
        it('should update the emoji correctly', async () => {
          const setEmoji = await profileActor.set_emoji({ emoji: '⚡️' });
          expect(setEmoji).toEqual(null);

          const getProfile = await profileActor.get_profile();
          expect(getProfile.emoji).toEqual('⚡️');
        });
      });

      describe('Set Avatar', () => {
        it('should update the avatar URL correctly', async () => {
          const setAvatar = await profileActor.set_avatar({ url: 'https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png' });
          expect(setAvatar).toEqual(null);

          const getProfile = await profileActor.get_profile();
          expect(getProfile.avatar).toEqual('https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png');
        });
      });

      describe('Set Banner', () => {
        it('should update the banner URL correctly', async () => {
          const setBanner = await profileActor.set_banner({ url: 'https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png' });
          expect(setBanner).toEqual(null);

          const getProfile = await profileActor.get_profile();
          expect(getProfile.banner).toEqual('https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png');
        });
      });

      describe('Set Profile', () => {
        it('should update the profile correctly', async () => {
          const metadata = {
            display_name: 'Barry Allen',
            description: 'The fastest man alive.',
            emoji: '⚡️',
            avatar: 'https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png',
            banner: 'https://static3.cbrimages.com/wordpress/wp-content/uploads/2020/07/the-flash-featured.jpg',
            version: 5,
          };
          const setProfile = await profileActor.set_profile(metadata);
          expect(setProfile).toEqual(null);

          const getProfile = await profileActor.get_profile();
          expect(getProfile).toEqual(metadata);
        });
      });
    });
  });

  describe('Special Cases', () => {
    it('The return should be set to null in case the profile does not exist', async () => {
      const getProfile = await profileActor.get_profile([fakePrincipal]);
      expect(getProfile).toEqual(null);
    });
  });
});
