# Interaction Guide

You can interact with the profile canister with the methods it provides. You can find all of the methods and their usage in the
shell script located [here](https://github.com/Psychedelic/dab/blob/main/scripts/method-tests.sh).

## Profile Canister Methods

The profile canister has seven methods. You can find the details of these methods in the [candid file](https://github.com/Psychedelic/dab/blob/main/candid/profile.did).

| Method Name        | Description                                                                                          |
| -----------        | -----------                                                                                          |
| get_profile        | This method returns the public information of the profile associated with the principal ID provided. |
| set_display_name   | This method updates the display name of the caller.                                                  |
| set_description    | This method updates the biography of the caller.                                                     |
| set_emoji          | This method updates the emoji associated with the caller.                                            |
| set_avatar         | This method updates the link to the avatar of the caller.                                            |
| set_banner         | This method updates the link to the banner of the caller.                                            |
| set_profile        | This method updates all of the caller's profile information together.                                |

## How to use them?

In this section we take a look at different methods and try to set-up a profile for ourselves with them. Let's start by adding a name to the profile:

```bash
$ dfx canister call profile set_display_name "(\"Peter Parker\")"
()
```

Now if we check our profile with the `get_profile` method, we can see that the display name is set to what we want it to be:

```bash
$ dfx canister call profile get_profile "(\"null\")"
(
  opt record {
    banner = null;
    description = null;
    emoji = null;
    display_name = opt "Peter Parker";
    version = 0;
    avatar = null;
  },
)
```

As you can see other fields are all set to `null`. That's because we haven't set them up, yet. You also might have noticed the `version` field. That field increments by one, everytime that you change your profile information. Other dapps can use this field to cache the profile information and check for newer versions.

Let's add new information to our profile now:

```bash
$ dfx canister call profile set_description "(\"Your friendly neighbourhood Spider-Man\")"
()
$ dfx canister call profile set_avatar "(\"https://upload.wikimedia.org/wikipedia/en/2/21/Web_of_Spider-Man_Vol_1_129-1.png\")"
()
$ dfx canister call profile set_banner "(\"https://www.nme.com/wp-content/uploads/2020/09/Spider-Man-Suit.jpg\")"
()
$ dfx canister call profile set_emoji "(\"🕷\")"
()
$ dfx canister call profile get_profile "(\"null\")"
(
  opt record {
    banner = opt "https://www.nme.com/wp-content/uploads/2020/09/Spider-Man-Suit.jpg";
    description = opt "Your friendly neighbourhood Spider-Man";
    emoji = opt "🕷";
    display_name = opt "Peter Parker";
    version = 4;
    avatar = opt "https://upload.wikimedia.org/wikipedia/en/2/21/Web_of_Spider-Man_Vol_1_129-1.png";
  },
)
```

Looks like we have completed the profile, now! This canister also provides another method that let's you update all of the information instantly with just one call to the canister. This method is most efficient when you want to update more than one of your profile fields:

```bash
$ dfx canister call profile set_profile "(record {display_name= \"Barry Allen\"; description= \"The fastest man alive.\"; emoji= \"⚡️\"; avatar= \"https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png\"; banner= \"https://static3.cbrimages.com/wordpress/wp-content/uploads/2020/07/the-flash-featured.jpg\"; version= 5})"
()
$ dfx canister call profile get_profile "(\"null\")"
(
  opt record {
    banner = opt "https://static3.cbrimages.com/wordpress/wp-content/uploads/2020/07/the-flash-featured.jpg";
    description = opt "The fastest man alive.";
    emoji = opt "⚡️";
    display_name = opt "Barry Allen";
    version = 5;
    avatar = opt "https://upload.wikimedia.org/wikipedia/en/3/3b/Flash_%28Barry_Allen_circa_2019%29.png";
  },
)
```

And that's it! We have used all of our profile methods and with them we set-up our own profile. 